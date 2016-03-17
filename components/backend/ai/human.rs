use super::*;

enum HumanState {
    Idle,
    Explore,
}

pub struct Human {
    state: HumanState,
}

impl Human {
    fn new() -> Self {
        Human {
            state: HumanState::Idle,
        }
    }
}
