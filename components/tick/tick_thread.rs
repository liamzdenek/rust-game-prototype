use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use tick_traits::tick_thread::*;
use common::select2;

pub trait TickThreadFactory {
    fn new() -> Self;
}

impl TickThreadFactory for TickThread {
    fn new() -> TickThread {
        let (tx, rx) = channel();
        thread::Builder::new().name("TickThread".to_string()).spawn(move || {
            TickManager::new(rx).start();
        });
        tx
    }
}

struct TickManager {
    rx: Receiver<TickThreadMsg>,
}

impl TickManager {
    fn new(rx: Receiver<TickThreadMsg>) -> TickManager {
        TickManager{
            rx: rx,
        }
    }

    fn start(&mut self) {
        loop {
            let mut omsg: Option<_> = None;
            select2!{
                tmsg = self.rx => {
                    omsg = Some(tmsg);
                }
            };
           
            if let Some(msg) = omsg {
                if msg.is_err() {
                    return;
                }
                match msg.unwrap() {
                    TickThreadMsg::Register(tx) => {
                        self.register(tx);
                    }
                    TickThreadMsg::Exit => {
                        return;
                    }
                }
            }
        }
    }

    fn register(&mut self, tx: Sender<TickThreadEvent>) {

    }
}
