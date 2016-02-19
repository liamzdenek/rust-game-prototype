use std::sync::mpsc::{channel,Sender,Receiver};
use std::result;
use common::{ChanError};

pub type Result<T> = result::Result<T, Error>;
pub type TickThread = Sender<TickThreadMsg>;
pub type TickClient = Receiver<TickThreadEvent>;

pub enum TickThreadMsg {
    Register(Sender<TickThreadEvent>),
    Constrain(Receiver<()>),
    Exit
}

#[derive(Clone)]
pub enum TickThreadEvent {
    Tick,
    Exit,
}

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

#[derive(Clone)]
pub struct Tick {
    thread: TickThread,
}

impl Tick {
    pub fn new(thread: TickThread) -> Self {
        Tick{
            thread: thread,
        }
    }

    pub fn register(&self) -> Result<TickClient> {
        let (tx, rx) = channel();
        try!(send!(self.thread, TickThreadMsg::Register => (tx)));
        Ok(rx)
    }

    pub fn init_constraint(&self) -> Result<Sender<()>> {
        let (tx, rx) = channel();
        try!(send!(self.thread, TickThreadMsg::Constrain => (rx)));
        Ok(tx)
    }
}
