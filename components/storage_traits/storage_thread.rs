use std::sync::mpsc::{channel,Sender};
use common::{Cell, Position, GridKey, CellKey, DataKey,EntityData,ChanError};
use std::result;
use std::convert::From;

pub type Result<T> = result::Result<T, Error>;

pub type StorageThread = Sender<StorageThreadMsg>;

#[derive(Debug)]
pub enum Error {
    Unimplemented(&'static str),
    InternalParseError(String),
    NotFound(String),
    SendError(&'static str),
    RecvError(&'static str),
}

impl From<ChanError> for Error {
    fn from(err: ChanError) -> Error {
        match err {
            ChanError::SendError(err) => Error::SendError(err),
            ChanError::RecvError(err) => Error::RecvError(err),
        }
    }
}

pub enum StorageThreadMsg {
    GetCell(Sender<Result<Cell>>, Position),
    GetArea(Sender<Vec<(Position,Result<Cell>)>>, Position, Position),
    SetCell(Sender<Result<()>>, Position, Cell),
    GetRawPosDataByPosition(Sender<Result<(GridKey,CellKey)>>, Position),
    GetAllEntities(Sender<Vec<EntityData>>),
    Exit,
}

#[derive(Clone)]
pub struct Storage {
    thread: StorageThread,
}

impl Storage {
    pub fn new(thread: StorageThread) -> Self {
        Storage{
            thread: thread,
        }
    }

    pub fn get_area(&self, pos_1: Position, pos_2: Position) -> Result<Vec<(Position, Result<Cell>)>> {
        Ok(try!(req_rep!(self.thread, StorageThreadMsg::GetArea => (pos_1, pos_2))))
    }

    pub fn get_cell(&self, pos: Position) -> Result<Cell> {
        try!(req_rep!(self.thread, StorageThreadMsg::GetCell => (pos)))
    }

    pub fn set_cell(&self, pos: Position, pix: Cell) -> Result<()> {
        try!(req_rep!(self.thread, StorageThreadMsg::SetCell => (pos, pix)))
    }
    
    pub fn get_all_entities(&self) -> Result<Vec<EntityData>> {
        Ok(try!(req_rep!(self.thread, StorageThreadMsg::GetAllEntities => ())))
    }
}
