use rand;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use common::{EntityData,Position,EntityId,ChanError};
use backend_traits::entity_thread::*;
use backend_traits::environment_thread::Environment;

pub trait EntityThreadFactory {
    fn new(EntityData, EntityContext) -> Self;
}

impl EntityThreadFactory for EntityThread {
    fn new(data: EntityData, ctx: EntityContext) -> Self {
        let (tx, rx) = channel();
        thread::Builder::new().name("EntityThread".to_string()).spawn(move || {
            EntityManager::new(rx, data, ctx).start();
        });

        tx
    }
}

#[derive(Clone)]
pub struct EntityContext {
    pub environment: Environment,
}

pub struct EntityManager {
    rx: Receiver<EntityThreadMsg>,
    data: EntityData,
    ctx: EntityContext,
}

impl EntityManager {
    fn new(rx: Receiver<EntityThreadMsg>, data: EntityData, ctx: EntityContext) -> Self {
        EntityManager{
            rx: rx,
            data: data,
            ctx: ctx,
        }
    }

    fn start(&mut self) {
        loop {
            let val = self.rx.recv();
            match val.unwrap_or(EntityThreadMsg::Exit) {
                EntityThreadMsg::Tick(sender) => {
                    //println!("entity thread got tick");
                    let x_is_neg = rand::random::<bool>();
                    let y_is_neg = rand::random::<bool>();
                    let x_is_one = rand::random::<bool>();
                    let y_is_one = rand::random::<bool>();
                    let mut true_x = 0;
                    let mut true_y = 0;
                    if x_is_one { true_x += 1; }
                    if y_is_one { true_y += 1; }
                    if x_is_neg { true_x = -true_x; }
                    if y_is_neg { true_y = -true_y; }
                    let newpos = self.data.pos.clone().rel(true_x, true_y);
                    if newpos != self.data.pos {
                        self.emit_update_pos(sender, newpos).unwrap();
                    } else {
                        self.emit_idle(sender).unwrap();
                    }
                },
                EntityThreadMsg::News(many_news) => {
                    println!("entity thread got news: {:?}", many_news);
                    for news in many_news.into_iter() {
                        match news {
                            EntityThreadNews::LastTickEventFailed => {
                                // oops
                                println!("last tick event failed");
                            },
                            EntityThreadNews::UpdateEntityData(updates) => {
                                self.data.apply(updates); 
                            },
                        }
                    }
                },
                EntityThreadMsg::Exit => {
                    return;
                },
            }
        }
    }

    fn emit_update_pos(&mut self, sender: Sender<(EntityId, TickEvent)>, pos: Position) -> Result<()>{
        try!(
            sender.send((self.data.id, TickEvent::Move(pos)))
                .map_err(|e| {
                    ChanError::SendError("emit_update_pos")
                })
        );
        Ok(())
    }
    fn emit_idle(&mut self, sender: Sender<(EntityId, TickEvent)>) -> Result<()>{
        try!(
            sender.send((self.data.id, TickEvent::Idle))
                .map_err(|e| {
                    ChanError::SendError("emit_idle")
                })
        );
        Ok(())
    }
}
