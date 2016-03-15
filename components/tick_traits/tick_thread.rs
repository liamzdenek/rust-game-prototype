use std::sync::mpsc::{channel,Sender,Receiver};
use std::result;
use common::{ChanError};

pub type Result<T> = result::Result<T, Error>;
pub type TickThread = Sender<TickThreadMsg>;
pub type TickClient = Receiver<TickThreadEvent>;

pub enum TickThreadMsg {
    Register(Sender<TickThreadEvent>),
    Constrain(Receiver<()>),
    GetTickLength(Sender<u32>), // ms
    SetSpeed(u32), // ms
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


    pub fn set_speed(&mut self, ms: u32) -> Result<()> {
        try!(send!(self.thread, TickThreadMsg::SetSpeed => (ms)));
        Ok(())
    }   

    pub fn init_constraint(&self) -> Result<Sender<()>> {
        let (tx, rx) = channel();
        try!(send!(self.thread, TickThreadMsg::Constrain => (rx)));
        Ok(tx)
    }

    pub fn get_tick_length(&self) -> Result<u32> {
        let res = try!(req_rep!(self.thread, TickThreadMsg::GetTickLength => ()));
        Ok(res)
    }
}
