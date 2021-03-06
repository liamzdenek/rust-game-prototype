use backend_traits::storage_thread::{Storage,StorageThread,StorageThreadMsg,Result,Error};
use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use std::collections::HashMap;
use usage::Usage;
use common::{Cell, GridKey, DataKey, CellKey, Position,EntityData};
use rustc_serialize::{json, Decodable, Encodable};

pub trait StorageThreadFactory {
    fn new() -> Self;
}

impl StorageThreadFactory for StorageThread {
    fn new() -> StorageThread {
        let (tx, rx) = channel();
        thread::Builder::new().name("StorageThread".to_string()).spawn(move || {
            let backend = Box::new(MemoryStoragePrimitive::default());
            StorageManager::new(rx, backend).start();
        });

        tx
    }
}

pub trait GridFactory {
    fn gen(&mut self, grid_key: GridKey, size: (u64, u64)) -> StorageGridData;
}

pub struct DefaultMapFactory;

impl DefaultMapFactory {
    fn new() -> Self {
        DefaultMapFactory
    }
}

impl GridFactory for DefaultMapFactory {
    fn gen(&mut self, grid_key: GridKey, size: (u64, u64)) -> StorageGridData {
        let mut data = StorageGridData::new(size);
        for t_x in 0..size.0 {
            for t_y in 0..size.1 {
                let mut t_cell = Cell{
                    terrain: 0,
                    ground: 0,
                    structure_id: 0,
                    is_structure_center: false,
                };

                // checkerboard
                let (real_x, real_y) = (
                    grid_key.x * size.0 as i64 + t_x as i64,
                    grid_key.y * size.1 as i64 + t_y as i64,
                );
                if real_x % 10 == 0 && real_y % 10 == 0 || real_x == 0 || real_y == 0  {
                    t_cell.terrain = 1;
                }
                data.set_cell(CellKey{x: t_x, y: t_y}, t_cell);
            }
        }
        data
    }
}

#[test]
fn storage_thread_test() {
    let st = Storage::new(StorageThreadFactory::new());
    let pos = Position{ x: 0, y: 0};
    assert!(st.get_cell(pos).unwrap().terrain == "dirt");
    let pos2 = Position{ x: 0, y: 1};
    assert!(st.get_cell(pos2).unwrap().terrain == "sand");
}

trait StoragePrimitive {
    fn read(&mut self, file: String) -> Result<&String>;
    fn write(&mut self, file: String, data: String) -> Result<()>;
}

#[derive(Default)]
struct MemoryStoragePrimitive {
    data: HashMap<String,String>,
}

impl StoragePrimitive for MemoryStoragePrimitive {
    fn read(&mut self, file: String) -> Result<&String> {
        self.data.get(&file).ok_or(Error::NotFound(format!("File not found: {}",file)))
    }

    fn write(&mut self, file: String, data: String) -> Result<()> {
        Err(Error::Unimplemented("MemoryStoragePrimitive.write"))
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct StorageGridData {
    size: usize,
    data: Vec<Vec<Cell>>,
}

impl StorageGridData {
    fn new(size: (u64, u64)) -> Self {
        StorageGridData{
            size: size.1 as usize,
            data: Vec::with_capacity(size.0 as usize),
        }
    }
    fn get_cell(&self, key: CellKey) -> Result<Cell> {
        Ok(
            self.data.get(key.x as usize)
                .and_then(|row| row.get(key.y as usize))
                .and_then(|cell| Some(cell.clone()))
                .unwrap_or_default()
        )
    }
    fn set_cell(&mut self, key: CellKey, pix: Cell) -> Result<()> {
        while self.data.len() <= key.x as usize {
            self.data.push(Vec::with_capacity(self.size));
        }

        let row = self.data.get_mut(key.x as usize).unwrap();

        while row.len() <= key.y as usize {
            row.push(Cell::default());
        }

        row[key.y as usize] = pix;

        Ok(())
    }
}

struct StorageManager {
    cell_size: (u64, u64),
    rx: Receiver<StorageThreadMsg>,
    backend: Box<StoragePrimitive>,
    loaded: Usage<GridKey, StorageGridData>,
    grid_factory: Box<GridFactory>
}

impl StorageManager {
    fn new(rx: Receiver<StorageThreadMsg>, backend: Box<StoragePrimitive>) -> StorageManager {
        StorageManager{
            cell_size: (100, 100),
            rx: rx,
            backend: backend,
            loaded: Usage::new(1000),
            grid_factory: Box::new(DefaultMapFactory::new()),
        }
    }

    fn start(&mut self) {
        loop {
            let val = self.rx.recv();
            match val.unwrap_or(StorageThreadMsg::Exit) {
                StorageThreadMsg::GetCell(sender, position) => {
                    self.get_cell(sender, position);
                }
                StorageThreadMsg::GetArea(sender, pos_1, pos_2) => {
                    self.get_area(sender, pos_1, pos_2);
                }
                StorageThreadMsg::SetCell(sender, position, cell) => {
                    self.set_cell(sender, position, cell);
                }
                StorageThreadMsg::GetRawPosDataByPosition(sender, position) => {
                    sender.send(Err(Error::Unimplemented("GetRawPosDataByPosition")));
                }
                StorageThreadMsg::GetAllEntities(sender) => {
                    sender.send(vec![
                        EntityData{
                            id: 1,
                            pos: Position{ x: 0, y: 0 },
                            last_pos: Position{ x: 0, y: 0},
                            kind: "human".to_string(),
                            data: "{}".to_string(),
                            .. Default::default()
                        },
                    ]);
                }
                StorageThreadMsg::Exit => {
                    return;
                }
            }
        }
    }

    fn get_area(&mut self, sender: Sender<Vec<(Position, Cell)>>, pos_1: Position, pos_2: Position) {
        let size = ((pos_2.x - pos_1.x) * (pos_2.y - pos_1.y)) as usize;
        //println!("area size: {:?}", size);
        let mut ret = Vec::with_capacity(size);
        for t_x in pos_1.x..pos_2.x {
            for t_y in pos_1.y..pos_2.y {
                let tpos = Position{ x: t_x, y: t_y };
                let(grid_key, cell_key) = self.get_keys(tpos.clone());
                let data = self.get_grid_data(grid_key).get_cell(cell_key);
                if data.is_ok() {
                    ret.push((
                        tpos,
                        data.unwrap(),
                    ));
                }
            }
        }
        sender.send(ret);
    }

    fn get_cell(&mut self, sender: Sender<Result<Cell>>, pos: Position) {
        let (grid_key, cell_key) = self.get_keys(pos);
        let grid_cell = self.get_grid_data(grid_key);
        let res = grid_cell.get_cell(cell_key);
        sender.send(res);
    }

    fn set_cell(&mut self, sender: Sender<Result<()>>, pos: Position, pix: Cell) {
        let (grid_key, cell_key) = self.get_keys(pos);
        let mut grid_cell = self.get_grid_data(grid_key);
        let res = grid_cell.set_cell(cell_key, pix);
        sender.send(res);
    }

    fn get_keys(&mut self, pos: Position) -> (GridKey, CellKey) {
        let mut gridkey = GridKey{
            x: pos.x/(self.cell_size.0 as i64),
            y: pos.y/(self.cell_size.1 as i64),
        };
        if pos.x < 0 {
            gridkey.x -= 1;
        }
        if pos.y < 0 {
            gridkey.y -= 1;
        }
        let cellkey = CellKey{
            x: (pos.x - (gridkey.x * self.cell_size.0 as i64)).abs() as u64,
            y: (pos.y - (gridkey.y * self.cell_size.1 as i64)).abs() as u64,
        };
        (gridkey, cellkey)
    }

    fn get_grid_data(&mut self, grid_key: GridKey) -> &mut StorageGridData {
        if !self.loaded.contains_key(&grid_key) {
            let mut new = self._load(grid_key.clone()).unwrap_or_else(|err| {
                self.grid_factory.gen(grid_key.clone(), self.cell_size)
            });
            self.loaded.insert(grid_key.clone(),new);
        }
        self.loaded.get_mut(&grid_key).unwrap()
    }

    fn _load(&mut self, grid_key: GridKey) -> Result<StorageGridData> {
        let mut path = format!("map/x{}.y{}.json", grid_key.x, grid_key.y);
        let raw_str = try!(self.backend.read(path));
        json::decode(raw_str)
            .map_err(|e| Error::InternalParseError(format!("{}",e)))
    }
}
