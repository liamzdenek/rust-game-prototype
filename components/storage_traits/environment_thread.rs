use std::sync::mpsc::{channel,Sender,Receiver};
use std::result;
use common::{Position,Cell};
use super::entity_thread::Entity;

pub type Result<T> = result::Result<T, Error>;

pub type EnvironmentThread = Sender<EnvironmentThreadMsg>;

#[derive(Debug)]
pub enum Error {
    Unimplemented(&'static str),
    SendError(&'static str),
    RecvError(&'static str),
}

pub enum EnvironmentThreadMsg {
    RegisterEntity(Entity),
    Observe(Sender<Vec<(Position,Cell,Entity)>>, Position, u32), // u32 == range
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
}

