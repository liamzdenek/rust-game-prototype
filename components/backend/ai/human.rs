use super::*;
use rand;
use common::{EntityData};

enum HumanState {
    Idle,
    Explore,
}

pub struct Human {
    state: HumanState,
}

impl Human {
    pub fn new() -> Self {
        Human {
            state: HumanState::Idle,
        }
    }

    pub fn tick(&mut self, data: &mut EntityData, mut sender: EntityResponder) {
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
        let newpos = data.pos.clone().rel(true_x, true_y);
        if newpos != data.pos {
            sender.emit_update_pos(newpos).unwrap();
        } else {
            sender.emit_idle().unwrap();
        }
    }
}
