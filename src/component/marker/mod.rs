use game_engine::prelude::*;

mod basic;
mod ids;

pub use self::{
    basic::*,
    ids::*,
};

pub(super) fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<Loader>()
        .register_component::<Solid>()
        .register_component::<CameraTarget>()
        .register_component::<Pickup>()
        .pipe(ids::register)
}
