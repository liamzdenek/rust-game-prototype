use std::sync::mpsc::{channel,Sender,Receiver};
use std::time::Duration;
use std::thread;

pub enum TimerMsgs {
    EmitOnce(Duration),
    EmitFinite(Duration, u64),
    EmitForever(Duration),
    Exit,
}

pub enum TimerEvents {
    Emit,
}

pub struct Timer {
    erx: Receiver<TimerEvents>,
}

pub struct TimerController {
    ctx: Sender<TimerMsgs>,
}

impl Timer {
    fn new() -> (Self,TimerController) {
        let(ctx, crx) = channel(); // control tx/rx
        let(etx, erx) = channel(); // event tx/rx

        thread::Builder::new().name("TimerThread".to_string()).spawn(move || {
            TimerManager::new(etx,crx).start();
        });

        (Timer{
            erx: erx,
        },
        TimerController{
            ctx: ctx,
        })
    }
}

pub struct TimerManager {
    crx: Receiver<TimerMsgs>,
    etx: Sender<TimerEvents>,
}

#[test]
fn timer_test() {
    let (timer, controller) = Timer::new();
}

impl TimerManager {
    fn new(etx: Sender<TimerEvents>, crx: Receiver<TimerMsgs>) -> TimerManager {
        TimerManager{
            crx: crx,
            etx: etx,
        }
    }

    fn start(&mut self) {
        loop {
            let res_msg = self.crx.recv();
            if res_msg.is_err() || self.handle(res_msg.unwrap()) {
                return;
            }
        }
    }

    fn handle(&mut self, msg: TimerMsgs) -> bool {
        match msg {
            TimerMsgs::EmitOnce(duration) => {
                thread::sleep(duration);
                self.etx.send(TimerEvents::Emit);
                false
            }
            TimerMsgs::EmitFinite(duration, count) => {
                for _ in 0..count {
                    thread::sleep(duration);
                    self.etx.send(TimerEvents::Emit);
                    if self.check() { return false; }
                }
                false
            }
            TimerMsgs::EmitForever(duration) => {
                loop {
                    thread::sleep(duration);
                    self.etx.send(TimerEvents::Emit);
                    if self.check() { return false; }
                }
                false
            }
            TimerMsgs::Exit => {
                true
            }
        }
    }

    fn check(&mut self) -> bool {
        self.crx.try_recv().is_ok()
    }
}
