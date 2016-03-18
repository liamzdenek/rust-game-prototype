use super::*;
use rand;
use common::{EntityData,Position};
use super::mapmemory::{MapMemory,RoutingInstructions};
use std::collections::VecDeque;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone)]
enum HumanState {
    Idle,
    Explore(Position, RoutingInstructions),
}

pub struct StateStack<T> {
    stack: VecDeque<T>,
}

impl<T> StateStack<T> {
    pub fn new(init: T) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(init);
        StateStack{
            stack: stack,
        }
    }

    pub fn push(&mut self, new: T) {
        self.stack.push_back(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.stack.len() >= 2 {
            self.stack.pop_back()
        } else {
            None
        }
    }

    pub fn replace_current(&mut self, new: T) -> T {
        let ret = self.stack.pop_back();
        self.stack.push_back(new);
        ret.unwrap() // guaranteed to always exist
    }

    pub fn replace_first(&mut self, new: T) -> T {
        let ret = self.stack.pop_front();
        self.stack.push_front(new);
        ret.unwrap() //guaranteed to always exist
    }

    pub fn get_current(&mut self) -> &mut T {
        let len = self.stack.len() - 1;
        self.stack.get_mut(len).unwrap()
    }
}

pub struct Human {
    state: StateStack<HumanState>,
    map: MapMemory,
}

impl Human {
    pub fn new() -> Self {
        Human {
            state: StateStack::new(HumanState::Idle),
            map: MapMemory::new(),
        }
    }

    pub fn tick(&mut self, data: &mut EntityData, mut sender: EntityResponder) {
        let cur = self.state.get_current().clone();
        match cur {
            HumanState::Idle => {
                let between = Range::new(-10, 10);
                let mut rng = rand::thread_rng();
                let target_pos = data.pos.clone().rel(between.ind_sample(&mut rng), between.ind_sample(&mut rng));
                let route = self.map.get_path(data.pos.clone(), target_pos.clone());
                self.state.push(
                    HumanState::Explore(target_pos, route),
                );
            },
            HumanState::Explore(target_pos, routing_instructions) => {
                let next = routing_instructions.get_next(data.pos.clone());
                if next.is_none() {
                    self.state.pop();
                    return;
                }
                let next = next.unwrap();
                sender.emit_update_pos(next).unwrap();
            },
        }
    }

    pub fn get_memory(&mut self) -> &mut MapMemory {
        &mut self.map
    }
}
