use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use common::{EntityData};
use storage_traits::entity_thread::*;
use storage_traits::environment_thread::Environment;

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
    ctx: EntityContext,
}

impl EntityManager {
    fn new(rx: Receiver<EntityThreadMsg>, data: EntityData, ctx: EntityContext) -> Self {
        EntityManager{
            rx: rx,
            ctx: ctx,
        }
    }

    fn start(&mut self) {
        loop {
            let val = self.rx.recv();
            match val.unwrap_or(EntityThreadMsg::Exit) {
                EntityThreadMsg::Tick(sender) => {

                },
                EntityThreadMsg::Exit => {
                    return;
                },
            }
        }
    }
}
