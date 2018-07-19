use engine::prelude::*;

mod basic;
mod dialog;
pub use self::{
    basic::*,
    dialog::*,
};

pub(super) fn register(game: Game) -> Game {
    game.register_component::<Player>()
        .register_component::<Solid>()
        .register_component::<CameraTarget>()
        .register_component::<Dialog>()
}
