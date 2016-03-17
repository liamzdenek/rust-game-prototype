#[derive(Debug,Clone,Default,Eq,PartialEq,RustcEncodable,RustcDecodable)]
pub struct Cell {
    pub terrain: u64,
    pub ground: u64,
    pub structure_id: u64,
    pub is_structure_center: bool,
}

pub type EntityId = u64;

#[derive(Debug)]
pub enum EntityDataMutation {
    LastTickEventFailed,
    UpdatePosition(Position),
}

#[derive(Debug,Eq,PartialEq,Hash,Clone,Default)]
pub struct EntityData {
    pub id: EntityId,
    pub last_pos: Position,
    pub pos: Position,
    pub kind: String,
    pub data: String,
    pub last_tick_failed: bool,
}

impl EntityData {
    pub fn apply(&mut self, changes: Vec<EntityDataMutation>) {
        for change in changes.into_iter() {
            match change {
                EntityDataMutation::LastTickEventFailed => {
                    self.last_tick_failed = true;
                },
                EntityDataMutation::UpdatePosition(position) => {
                    self.last_pos = self.pos.clone();
                    self.pos = position;
                }
            }
        }
    }
}

#[derive(Debug,Eq,PartialEq,Hash,Clone,Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn rel(mut self, x: i64, y: i64) -> Position {
        self.x += x;
        self.y += y;
        self
    }

    pub fn neighbors(&self) -> Vec<(Position,usize)> {
        vec![
            // along axes
            (self.clone().rel(-1,  0), 1),
            (self.clone().rel( 1,  0), 1),
            (self.clone().rel( 0, -1), 1),
            (self.clone().rel( 0,  1), 1),

            // diagonally
            (self.clone().rel(-1, -1), 2),
            (self.clone().rel(-1,  1), 2),
            (self.clone().rel( 1, -1), 2),
            (self.clone().rel( 1,  1), 2),
        ]
    }
}

impl From<(i32, i32)> for Position {
    fn from(i: (i32, i32)) -> Position {
        Position{
            x: i.0 as i64,
            y: i.1 as i64,
        }
    }
}

#[derive(Eq,PartialEq,Hash,Clone)]
pub struct GridKey {
    pub x: i64,
    pub y: i64,
}

pub type DataKey = String; 

#[derive(Eq,PartialEq,Hash,Clone,RustcDecodable,RustcEncodable)]
pub struct CellKey {
    pub x: u64,
    pub y: u64,
}
