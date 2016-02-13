#[derive(Debug,Clone,Default,Eq,PartialEq,RustcEncodable,RustcDecodable)]
pub struct Cell {
    pub terrain: String,
    pub ground: String,
    pub structure_id: String,
    pub is_structure_center: bool,
}

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
pub struct EntityData {
    pub pos: Position,
    pub kind: String,
    pub data: String,
}

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
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
