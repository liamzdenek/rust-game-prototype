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
    tick_avg: u32,
    clients: Vec<Sender<TickThreadEvent>>,
    constraint: Option<Receiver<()>>,
}

enum PausedState {
    Unchanged,
    Paused,
    Unpaused,
}

impl TickManager {
    fn new(rx: Receiver<TickThreadMsg>) -> TickManager {
        TickManager{
            rx: rx,
            tick_avg: 500,
            timer: timer::periodic_ms(500),
            clients: vec![],
            constraint: None,
        }
    }


    // return = should exit
    fn handle_msg(&mut self, msg: result::Result<TickThreadMsg,RecvError>) -> (PausedState, bool) {
        let mut is_paused = PausedState::Unchanged;
        match msg.unwrap_or(TickThreadMsg::Exit) {
            TickThreadMsg::Constrain(rx) => {
                self.constraint = Some(rx);
            }
            TickThreadMsg::Register(tx) => {
                self.register(tx);
            },
            TickThreadMsg::GetTickLength(tx) => {
                tx.send(self.tick_avg);
            },
            TickThreadMsg::SetSpeed(ms) => {
                if ms == 0 {
                    is_paused = PausedState::Paused;
                } else {
                    is_paused = PausedState::Unpaused;
                    self.timer = timer::periodic_ms(ms);
                }
            },
            TickThreadMsg::Exit => {
                return (is_paused, true);
            }
        }
        return (is_paused, false);
    }

    fn start(&mut self) {
        let mut is_paused = false;
        loop {
            if !is_paused {
                select2!{
                    tmsg = self.rx => {
                        let (new_state, do_exit) = self.handle_msg(tmsg);

                        if do_exit {
                            return;
                        }

                        match new_state {
                            PausedState::Unchanged => {},
                            PausedState::Paused => { is_paused = true; }
                            PausedState::Unpaused => { is_paused = false; }
                        }
                    },
                    msg = self.timer => {
                        self.emit(TickThreadEvent::Tick);
                    },
                };
            } else {
                select2!{
                    tmsg = self.rx => {
                        let (new_state, do_exit) = self.handle_msg(tmsg);

                        if do_exit {
                            return;
                        }

                        match new_state {
                            PausedState::Unchanged => {},
                            PausedState::Paused => { is_paused = true; }
                            PausedState::Unpaused => { is_paused = false; }
                        }
                    },
                };
            }
        }
    }

    fn emit(&mut self, msg: TickThreadEvent) {
        //println!("emitting tick");
        for client in self.clients.iter() {
            client.send(msg.clone());
        }
        let is_err = self.constraint.as_ref().and_then(|rx| {
            //println!("waiting on constraint to emit next tick");
            Some(rx.recv().is_err())
        }).unwrap_or(false);
        if is_err {
            //println!("constraint expired, unbinding");
            self.constraint = None;
        };
    }

    fn register(&mut self, tx: Sender<TickThreadEvent>) {
        println!("got register client");
        self.clients.push(tx);
    }
}
