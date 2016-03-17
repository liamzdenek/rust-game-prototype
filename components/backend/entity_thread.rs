use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use common::{EntityData,Position,EntityId,ChanError};
use backend_traits::entity_thread::*;
use backend_traits::environment_thread::Environment;
use ai::EntityResponder;
use ai::human::Human;

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
        let mut human = Human::new();
        let mut last_tick_failed = false;
        loop {
            let val = self.rx.recv();
            match val.unwrap_or(EntityThreadMsg::Exit) {
                EntityThreadMsg::Tick(sender) => {
                    let id = self.data.id.clone();
                    human.tick(&mut self.data, EntityResponder{
                        id: id,
                        sender: sender,
                        already_sent: false,
                    });
                    self.data.last_tick_failed = false;
                },
                EntityThreadMsg::News(many_news) => {
                    println!("entity thread got news: {:?}", many_news);
                    for news in many_news.into_iter() {
                        match news {
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
}
