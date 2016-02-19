#[derive(Debug,Clone,Default,Eq,PartialEq,RustcEncodable,RustcDecodable)]
pub struct Cell {
    pub terrain: String,
    pub ground: String,
    pub structure_id: String,
    pub is_structure_center: bool,
}

pub type EntityId = u64;

#[derive(Debug)]
pub enum EntityDataMutation {
    UpdatePosition(Position),
}

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
pub struct EntityData {
    pub id: EntityId,
    pub pos: Position,
    pub kind: String,
    pub data: String,
}

impl EntityData {
    pub fn apply(&mut self, changes: Vec<EntityDataMutation>) {
        for change in changes.into_iter() {
            match change {
                EntityDataMutation::UpdatePosition(position) => {
                    self.pos = position;
                }
            }
        }
    }
}

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
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
