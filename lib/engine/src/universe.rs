use std::collections::HashMap;
use physics;
use rustc_serialize::{json, Decodable, Encodable};
use usage::Usage;
use std::io::prelude::*;
use std::fs;
use std::str;
use std::result;

pub type Result<T> = result::Result<T, UniverseError>;

pub struct Universe {
    pub grid: Grid<DiskStorageEngine>,
}

impl Universe {
    pub fn new(grid: Grid<DiskStorageEngine>) -> Self {
        Universe{
            grid: grid,
        }
    }

    pub fn get_json<D: Decodable>(&mut self, filename: String) -> Result<D> {
        self.grid.get_json(filename)
    }

    pub fn set_json<D: Encodable>(&mut self, filename: String, data: &D) -> Result<()> {
        self.grid.set_json(filename, data)
    }
    
    pub fn get_pixel(&mut self, x: i64, y: i64) -> Result<Pixel> {
        self.grid.get_pixel(x, y)
    }

    pub fn set_pixel(&mut self, x: i64, y: i64, pixel: Pixel) -> Result<()> {
        self.grid.set_pixel(x,y,pixel)
    }
    /*
    pub fn run_forever<P: physics::Physics>(&mut self, physics: &mut P) {
        loop {
            self.run_tick(physics);
        }
    }

    pub fn run_tick<P: physics::Physics>(&mut self, physics: &mut P) {
        physics.run_tick(self);
    }
    */
}

#[derive(Eq,PartialEq,Hash,Clone,RustcDecodable,RustcEncodable)]
pub struct GridKey {
    x: i64,
    y: i64,
}

pub struct Grid<T: StorageEngine> {
    tile_width: u32,
    tile_height: u32,
    loaded: Usage<GridKey, Box<StorageCell>>,
    se: T,
}

impl<T: StorageEngine> Grid<T> {
    pub fn new(se: T) -> Self {
        Grid{
            tile_width: 10,
            tile_height: 10,
            loaded: Usage::new(1000),
            se: se,
        }
    }

    pub fn get_pixel(&mut self, x: i64, y: i64) -> Result<Pixel> {
        let args = self.derive_pos(x, y);
        
        let storage = try!(self.get_storage_cell(args.2));

        Ok(try!(storage.get_pixel(args.0, args.1)))
    }
    
    pub fn set_pixel(&mut self, x: i64, y: i64, pixel: Pixel) -> Result<()> {
        let args = self.derive_pos(x,y);

        let storage = try!(self.get_storage_cell(args.2));

        try!(storage.set_pixel(args.0, args.1, pixel));
        Ok(())
    }

    pub fn derive_pos(&self, x: i64, y: i64) -> (u64,u64,GridKey) {
        let key = GridKey{
            x: x/(self.tile_width as i64),
            y: y/(self.tile_height as i64)
        };
        (
            (x - (key.x * self.tile_width as i64)).abs() as u64,
            (y - (key.y * self.tile_height as i64)).abs() as u64,
            key
        )
    }

    pub fn get_storage_cell(&mut self, key: GridKey) -> Result<&mut Box<StorageCell>> {
        if !self.loaded.contains_key(&key) {
            let mut new = try!(self.se.load(key.clone())); 

            self.loaded.insert(key.clone(), new);
        }
        let data = self.loaded.get_mut(&key);
        if data.is_none() {
            return Err(UniverseError::GridNotLoaded);
        }
        return Ok(data.unwrap());
    }

    pub fn get_json<D: Decodable>(&mut self, filename: String) -> Result<D> {
        self.se.get_json(filename)
    }

    pub fn set_json<D: Encodable>(&mut self, filename: String, data: &D) -> Result<()> {
        self.se.set_json(filename, data)
    }
}

pub struct DiskStorageEngineParams {
    pub dir: String,
}

pub struct DiskStorageEngine {
    params: DiskStorageEngineParams,
}

#[derive(RustcDecodable,RustcEncodable)]
pub struct DiskStorageCell {
    data: HashMap<CellKey, Pixel>,
}

#[derive(Eq,PartialEq,Hash,Clone,RustcDecodable,RustcEncodable)]
pub struct CellKey {
    x: u64,
    y: u64
}

impl DiskStorageCell {
    pub fn new() -> Self {
        DiskStorageCell{
            data: HashMap::new(),
        }
    }
}

impl DiskStorageEngine {
    pub fn new(params: DiskStorageEngineParams) -> Self {
        DiskStorageEngine{
            params: params,    
        }
    }
}

impl StorageEngine for DiskStorageEngine {
    fn get_json<T: Decodable>(&self, filename: String) -> Result<T> {
        let mut path = self.params.dir.clone();
        path.push_str(&filename);
        let mut file = try!(fs::File::open(path));
        let mut string = vec![];
        try!(file.read_to_end(&mut string));
        let mut string = try!(str::from_utf8(string.as_slice()));
        Ok(try!(json::decode(&string)))
    }

    fn set_json<T: Encodable>(&self, filename: String, data: &T) -> Result<()> {
        try!(fs::create_dir(self.params.dir.clone()));
        let mut path = self.params.dir.clone();
        path.push_str(&filename);
        let mut file = try!(fs::File::create(path));
        let mut string = try!(json::encode(data));
        try!(file.write_all(string.into_bytes().as_slice()));
        Ok(())
    }

    fn load(&mut self, k: GridKey) -> Result<Box<StorageCell>> {
        self.get_json(format!("chunk_{}_{}.json",k.x,k.y))
           .or_else(|e| Ok(DiskStorageCell::new()))
           .and_then(|v| {
                let mut mybox: Box<StorageCell> = Box::new(v);
                Ok(mybox)
           })
    }
}

impl StorageCell for DiskStorageCell {
    fn get_pixel(&self, x: u64, y: u64) -> Result<Pixel> {
        let key = CellKey{
            x: x,
            y: y,
        };
        Ok(
            self.data.get(&key)
                .map_or_else(
                    || Pixel::new("".to_string()),
                    |pixel| pixel.clone().to_owned(),
                )
        )
    }
    fn set_pixel(&mut self, x: u64, y: u64, pixel: Pixel) -> Result<()> {        
        let key = CellKey{
            x: x,
            y: y,
        };
        self.data.insert(key, pixel);
        Ok(())
        /*self.data.get(key)
            .or_else(|| {
                self.data[y as usize] = vec![];
                self.data.get(y as usize)
            })
            .and_then(|row| {
                for i in row.len()..x as usize {
                    row[i as usize] = Pixel::new("".to_string())
                }
                row.get_mut(x as usize)
            })
            .and_then(|cell| {
                cell = &mut pixel;
                Some(())
            })
            .ok_or(UniverseError::Unimplemented("internal error in set_pixel"))
        */
    }
    fn save(self) -> Result<()> {
        Err(UniverseError::Unimplemented("DiskStorageCell.save"))
    }
}

#[derive(RustcEncodable,RustcDecodable,Clone,Debug)]
pub struct Pixel {
    kind: String,
    data: String,
}

impl Pixel {
    pub fn new(kind: String) -> Self {
        Pixel{
            kind: kind,
            data: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum UniverseError {
    IOError(::std::io::Error),
    JsonDecoderError(json::DecoderError),
    JsonEncoderError(json::EncoderError),
    Utf8Error(str::Utf8Error),
    GridNotLoaded,
    Unimplemented(&'static str)
}

impl From<::std::io::Error> for UniverseError {
    fn from(err: ::std::io::Error) -> UniverseError {
        UniverseError::IOError(err)
    }
}

impl From<json::DecoderError> for UniverseError {
    fn from(err: json::DecoderError) -> UniverseError {
        UniverseError::JsonDecoderError(err)
    }
}

impl From<json::EncoderError> for UniverseError {
    fn from(err: json::EncoderError) -> UniverseError {
        UniverseError::JsonEncoderError(err)
    }
}

impl From<str::Utf8Error> for UniverseError {
    fn from(err: str::Utf8Error) -> UniverseError {
        UniverseError::Utf8Error(err)
    }
}

pub trait StorageEngine{
    fn load(&mut self, k: GridKey) -> Result<Box<StorageCell>>;
    fn get_json<T: Decodable>(&self, filename: String) -> Result<T>;
    fn set_json<T: Encodable>(&self, filename: String, data: &T) -> Result<()>;
}

pub trait StorageCell {
    fn save(self) -> Result<()>;
    fn get_pixel(&self, x: u64, y: u64) -> Result<Pixel>;
    fn set_pixel(&mut self, x: u64, y: u64, pixel: Pixel) -> Result<()>;
}
