use std::sync::mpsc::{channel,Sender,Receiver};
use super::environment_thread::Environment;
use std::result;
use common::{EntityId,Position};

pub type Result<T> = result::Result<T, Error>;
pub type EntityThread = Sender<EntityThreadMsg>;

#[derive(Debug)]
pub enum Error {
    UnknownEntityKind(String),
    Unimplemented(&'static str),
    SendError(&'static str),
    RecvError(&'static str),
}


pub enum EntityThreadMsg {
    Tick(Sender<(EntityId, TickEvent)>),
    Exit,
}

pub enum TickEvent {
    Idle,
    Move(Position)
}


pub struct Entity {
    rx: EntityThread
}

impl Entity {
    pub fn new(rx: EntityThread) -> Self {
        Entity{ rx: rx }
    }
}
