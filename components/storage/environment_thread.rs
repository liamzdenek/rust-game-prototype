use std::sync::mpsc::{channel,Sender,Receiver};
use storage_traits::environment_thread::{EnvironmentThreadMsg,EnvironmentThread};
//use super::storage_thread::Storage;
use storage_traits::storage_thread::{Storage,StorageThreadMsg};
use std::thread;

pub trait EnvironmentThreadFactory {
    fn new(storage: Storage) -> Self;
}

impl EnvironmentThreadFactory for EnvironmentThread {
    fn new(storage: Storage) -> EnvironmentThread {
        let (tx, rx) = channel();
        thread::Builder::new().name("EnvironmentThread".to_string()).spawn(move || {
            EnvironmentManager::new(rx, storage).start();
        });

        tx
    }
}

pub struct EnvironmentManager {
    rx: Receiver<EnvironmentThreadMsg>,
    storage: Storage,
}

impl EnvironmentManager {
    fn new(rx: Receiver<EnvironmentThreadMsg>, storage: Storage) -> Self {
        EnvironmentManager{
            storage: storage,
            rx: rx,
        }
    }

    fn init(&mut self) {
        let ents = self.storage.get_all_entities();
        println!("Entities: {:?}", ents); 
    }

    fn start(&mut self) {
        self.init();
        loop {
            let val = self.rx.recv();
            match val.unwrap_or(EnvironmentThreadMsg::Exit) {
                EnvironmentThreadMsg::RegisterEntity(entity) => {
                    
                },
                EnvironmentThreadMsg::Observe(sender, position, range) => {
                    
                },
                EnvironmentThreadMsg::Exit => {
                    return;
                }
            }
        }
    }
}
