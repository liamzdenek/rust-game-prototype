use std::sync::mpsc::{channel,Sender,Receiver};
use storage_traits::environment_thread::{EnvironmentThreadMsg,EnvironmentThread,Environment};
//use super::storage_thread::Storage;
use storage_traits::storage_thread::{Storage,StorageThreadMsg};
use storage_traits::entity_thread::{Entity};
use super::entity_thread::{EntityThreadFactory,EntityContext};
use tick_traits::tick_thread::{Tick,TickClient};
use std::thread;

pub trait EnvironmentThreadFactory {
    fn new(storage: Storage, tick: Tick) -> Self;
}

impl EnvironmentThreadFactory for EnvironmentThread {
    fn new(storage: Storage, tick: Tick) -> EnvironmentThread {
        let (tx, rx) = channel();
        let env = Environment::new(tx.clone());
        thread::Builder::new().name("EnvironmentThread".to_string()).spawn(move || {
            EnvironmentManager::new(rx, storage, tick, env).start();
        });

        tx
    }
}

pub struct EnvironmentManager {
    own_env: Environment,
    rx: Receiver<EnvironmentThreadMsg>,
    tick_client: TickClient,
    storage: Storage,
    entities: Vec<Entity>,
}

impl EnvironmentManager {
    fn new(rx: Receiver<EnvironmentThreadMsg>, storage: Storage, tick: Tick, env: Environment) -> Self {
        EnvironmentManager{
            own_env: env,
            storage: storage,
            rx: rx,
            tick_client: tick.register().unwrap(),
            entities: vec![],
        }
    }

    fn init(&mut self) {
        let ctx = EntityContext{
            environment: self.own_env.clone(),
        };
        for ent in self.storage.get_all_entities().unwrap().into_iter() {
            println!("Entity: {:?}", ent); 
            let tent = Entity::new(EntityThreadFactory::new(ent, ctx.clone()));
            self.entities.push(tent);
        }
    }

    fn start(&mut self) {
        self.init();
        loop {
            select2!{
                ev = self.tick_client => {
                    match ev {
                        Tick => {
                            println!("environment got tick");
                        }
                    }
                },
                val = self.rx => {
                    match val.unwrap_or(EnvironmentThreadMsg::Exit) {
                        EnvironmentThreadMsg::RegisterEntity(entity) => {
                            
                        },
                        EnvironmentThreadMsg::Observe(sender, position, range) => {
                            
                        },
                        EnvironmentThreadMsg::Exit => {
                            return;
                        }
                    }
                },
            }
        }
    }
}
