use game_engine::prelude::*;

mod move_path;
pub use self::{
    move_path::MovePath,
};

pub(super) fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<MovePath>()
}
