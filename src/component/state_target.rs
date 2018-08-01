use engine::prelude::*;
use resource::state::MainState;

#[derive(Component, Copy, Clone, Debug)]
pub struct StateTarget(pub MainState);

impl StateTarget {
    pub const fn new(state: MainState) -> Self {
        StateTarget(state)
    }
}
