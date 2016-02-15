use std::sync::mpsc::{channel,Sender,Receiver,RecvError};
use std::result;
use std::thread;
use tick_traits::tick_thread::*;
use common::select2;
use timer;


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
    timer: Receiver<()>,
    clients: Vec<Sender<TickThreadEvent>>,
}

impl TickManager {
    fn new(rx: Receiver<TickThreadMsg>) -> TickManager {
        TickManager{
            rx: rx,
            timer: timer::periodic_ms(1000),
            clients: vec![],
        }
    }


    // return = should exit
    fn handle_msg(&mut self, msg: result::Result<TickThreadMsg,RecvError>) -> bool {
        if msg.is_err() {
            return true;
        }
        match msg.unwrap() {
            TickThreadMsg::Register(tx) => {
                self.register(tx);
            },
            TickThreadMsg::Exit => {
                return true;
            }
        }
        return false;
    }

    fn start(&mut self) {
        loop {
            select2!{
                tmsg = self.rx => {
                    if self.handle_msg(tmsg) { return; }
                },
                msg = self.timer => {
                    self.emit(TickThreadEvent::Tick);
                },
            };
        }
    }

    fn emit(&mut self, msg: TickThreadEvent) {
        println!("emitting tick");
        for client in self.clients.iter() {
            client.send(msg.clone());
        }
    }

    fn register(&mut self, tx: Sender<TickThreadEvent>) {
        println!("got register client");
        self.clients.push(tx);
    }
}
