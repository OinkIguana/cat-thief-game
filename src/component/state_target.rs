use specs_derive::Component;
use game_engine::prelude::*;
use crate::resource::state::MainState;

#[derive(Component, Copy, Clone, Debug)]
pub struct StateTarget(pub MainState);

impl StateTarget {
    pub const fn new(state: MainState) -> Self {
        StateTarget(state)
    }
}
