use std::sync::mpsc::{channel,Sender,Receiver};
use backend_traits::environment_thread::{EnvironmentThreadMsg,EnvironmentThread,Environment,LocalEntityData};
//use super::storage_thread::Storage;
use backend_traits::storage_thread::{Storage,StorageThreadMsg};
use backend_traits::entity_thread::{Entity,TickEvent,EntityThreadNews};
use super::entity_thread::{EntityThreadFactory,EntityContext};
use tick_traits::tick_thread::{Tick,TickClient,TickThreadEvent};
use std::thread;
use common::{EntityId,Position,EntityDataMutation};

pub trait EnvironmentThreadFactory {
    fn new(backend: Storage, tick: Tick) -> Self;
}

impl EnvironmentThreadFactory for EnvironmentThread {
    fn new(backend: Storage, tick: Tick) -> EnvironmentThread {
        let (tx, rx) = channel();
        let env = Environment::new(tx.clone());
        thread::Builder::new().name("EnvironmentThread".to_string()).spawn(move || {
            EnvironmentManager::new(rx, backend, tick, env).start();
        });

        tx
    }
}

#[derive(Default)]
pub struct EnvironmentState {
    entities: Vec<LocalEntityData>,
}

impl EnvironmentState {
    fn apply_tick_events(&mut self, all_events: Vec<(EntityId, TickEvent)>) -> Vec<(EntityId, Vec<EntityThreadNews>)> {
        //println!("applying events: {:?}", all_events);
        let mut output = vec![];
        for (id,event) in all_events.into_iter() {
            match event {
                TickEvent::Idle => {},
                TickEvent::Move(ref position) => {
                    // if the position would conflict
                    let conflict = (0..self.entities.len()).find(|&ofs| self.entities[ofs].pos == position.clone().to_owned());
                    if let Some(tent) = conflict {
                        output.push((id, vec![EntityThreadNews::LastTickEventFailed]));
                    } else {
                        let updating = self.entities.iter_mut().find(|tent| tent.id == id).unwrap();
                        updating.last_pos = updating.pos.clone();
                        updating.pos = position.clone().to_owned();
                        output.push((id, vec![
                            EntityThreadNews::UpdateEntityData(vec![
                                EntityDataMutation::UpdatePosition(position.clone().to_owned())
                            ])
                        ]));
                    }
                }
            }
        }
        println!("events applied. Generated output: {:?}", output);
        output
    }
}

pub struct EnvironmentManager {
    own_env: Environment,
    rx: Receiver<EnvironmentThreadMsg>,
    tick: Tick,
    tick_client: TickClient,
    tick_constraint: Sender<()>,
    backend: Storage,
    state: EnvironmentState,
}

impl EnvironmentManager {
    fn new(rx: Receiver<EnvironmentThreadMsg>, backend: Storage, tick: Tick, env: Environment) -> Self {
        EnvironmentManager{
            own_env: env,
            backend: backend,
            rx: rx,
            tick_client: tick.register().unwrap(),
            tick_constraint: tick.init_constraint().unwrap(),
            tick: tick,
            state: EnvironmentState::default(),
        }
    }

    fn init(&mut self) {
        let ctx = EntityContext{
            environment: self.own_env.clone(),
        };
        for ent in self.backend.get_all_entities().unwrap().into_iter() {
            println!("Entity: {:?}", ent); 
            let tent = Entity::new(EntityThreadFactory::new(ent.clone(), ctx.clone()));
            let ent_data = LocalEntityData{
                id: ent.id,
                ent: tent,
                pos: ent.pos.clone(),
                last_pos: ent.pos,
            };
            self.state.entities.push(ent_data);
        }
    }

    fn emit_tick(&mut self) {
        //println!("environment got tick");
        let (tx, rx) = channel();
        let mut pending = vec![];
        let mut all_events = vec![];
        
        for entity_data in self.state.entities.iter() {
            entity_data.ent.tick(tx.clone()).unwrap();
            pending.push(entity_data.id.clone());
        }

        'recvloop: loop {
            if pending.len() == 0 {
                break;
            }

            select2_timeout!(
                5000 => {
                    println!("Entity threads timed out for this tick. Assuming idle and continuing. Skipped entities: {:?}", pending);
                    for id in pending {
                        all_events.push((id, TickEvent::Idle));
                    }
                    break 'recvloop;
                },
                msg = rx => {
                    if msg.is_err() {
                        break 'recvloop;
                    }
                    let (id, event) = msg.unwrap();
                    pending = pending.into_iter().filter(|tid| !(id == *tid)).collect(); 
                    //println!("got event from id: {:?}, {:?}", id, event);
                    all_events.push((id, event));
                },
            ); 
        }

        let updates = self.state.apply_tick_events(all_events);

        self.deliver_updates(updates);

        self.tick_constraint.send(());
    }

    fn deliver_updates(&mut self, updates: Vec<(EntityId, Vec<EntityThreadNews>)>) {
        for (id,news) in updates.into_iter() {
            let search = self.state.entities.iter().find(|tent| tent.id == id);
            if let Some(entitydata) = search {
                entitydata.ent.news(news);
            }
        }
    } 

    fn get_entities_by_area(&mut self, sender: Sender<Vec<LocalEntityData>>, pos1: Position, pos2: Position) {
        let mut pos1 = pos1;
        let mut pos2 = pos2;
        if pos1.x > pos2.x {
            let swp = pos1.x;
            pos1.x = pos2.x;
            pos2.x = swp;
        }
        if pos1.y > pos2.y {
            let swp = pos1.y;
            pos1.y = pos2.y;
            pos2.y = swp;
        }
        let pos1 = pos1;
        let pos2 = pos2;
        let result = self.state.entities.clone().into_iter().filter(|tent| {
            pos1.x.le(&tent.pos.x) && pos2.x.ge(&tent.pos.x) &&
            pos1.y.le(&tent.pos.y) && pos2.y.ge(&tent.pos.y)
        }).collect();
        sender.send(result);
    }

    fn start(&mut self) {
        self.init();
        loop {
            select2!{
                ev = self.tick_client => {
                    match ev.unwrap_or(TickThreadEvent::Exit) {
                        TickThreadEvent::Tick => {
                            self.emit_tick();
                        },
                        TickThreadEvent::Exit => {
                            panic!("environment got tick thread exit");
                        },
                    }
                },
                val = self.rx => {
                    match val.unwrap_or(EnvironmentThreadMsg::Exit) {
                        EnvironmentThreadMsg::GetEntitiesByArea(sender, pos1, pos2) => {
                            self.get_entities_by_area(sender, pos1, pos2);
                        },
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
