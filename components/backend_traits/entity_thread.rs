use std::sync::mpsc::{channel,Sender,Receiver};
use super::environment_thread::Environment;
use std::result;
use common::{EntityId,Position,ChanError,EntityDataMutation};

pub type Result<T> = result::Result<T, Error>;
pub type EntityThread = Sender<EntityThreadMsg>;

#[derive(Debug)]
pub enum Error {
    UnknownEntityKind(String),
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

pub enum EntityThreadMsg {
    Tick(Sender<(EntityId, TickEvent)>),
    News(Vec<EntityThreadNews>),
    Exit,
}

#[derive(Debug)]
pub enum EntityThreadNews {
    LastTickEventFailed,
    UpdateEntityData(Vec<EntityDataMutation>),
}

#[derive(Debug)]
pub enum TickEvent {
    Idle,
    Move(Position)
}

#[derive(Debug,Clone)]
pub struct Entity {
    tx: EntityThread
}

impl Entity {
    pub fn new(tx: EntityThread) -> Self {
        Entity{ tx: tx }
    }

    pub fn tick(&self, tx: Sender<(EntityId, TickEvent)>) -> Result<()> {
        try!(send!(self.tx, EntityThreadMsg::Tick => (tx)));
        Ok(())
    }

    pub fn news(&self, news: Vec<EntityThreadNews>) -> Result<()> {
        try!(send!(self.tx, EntityThreadMsg::News => (news)));
        Ok(())
    }
}
