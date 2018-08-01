//! The state of the player's progress through the game

mod main;

pub use self::{
    main::MainState,
};

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct State {
    pub main: MainState,
}
