use engine::prelude::*;

mod move_path;
pub use self::{
    move_path::MovePath,
};

pub(super) fn register(game: Game) -> Game {
    game.register_component::<MovePath>()
}
