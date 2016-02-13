use std::sync::mpsc::{channel,Sender};

pub type TickThread = Sender<TickThreadMsg>;

pub enum TickThreadMsg {
    Register(Sender<TickThreadEvent>),
    Exit
}

pub enum TickThreadEvent {
    Tick
}

pub struct Tick {
    thread: TickThread,
}

impl Tick {
    pub fn new(thread: TickThread) -> Self {
        Tick{
            thread: thread,
        }
    }
}
