use std::sync::mpsc::{channel,Sender,Receiver};
use std::result;
use common::{Position,Cell,EntityId,ChanError};
use super::entity_thread::Entity;

pub type Result<T> = result::Result<T, Error>;

pub type EnvironmentThread = Sender<EnvironmentThreadMsg>;

#[derive(Debug)]
pub enum Error {
    Unimplemented(&'static str),
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

#[derive(Debug,Clone)]
pub struct LocalEntityData {
    pub id: EntityId,
    pub pos: Position,
    pub ent: Entity,
}

pub enum EnvironmentThreadMsg {
    RegisterEntity(Entity),
    Observe(Sender<Vec<(Position,Cell,Entity)>>, Position, u32), // u32 == range
    GetEntitiesByArea(Sender<Vec<LocalEntityData>>, Position, Position),
    Exit,
}

#[derive(Clone)]
pub struct Environment {
    thread: EnvironmentThread,
}

impl Environment {
    pub fn new(thread: EnvironmentThread) -> Self {
        Environment{
            thread: thread,
        }
    }

    pub fn get_entities_by_area(&self, pos1: Position, pos2: Position) -> Result<Vec<LocalEntityData>> {
        Ok(try!(req_rep!(self.thread, EnvironmentThreadMsg::GetEntitiesByArea => (pos1, pos2))))
    }
}

