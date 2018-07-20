use engine::prelude::*;

mod basic;
pub use self::{
    basic::*,
};

pub(super) fn register(game: Game) -> Game {
    game.register_component::<Player>()
        .register_component::<Solid>()
        .register_component::<CameraTarget>()
}
