use engine::prelude::*;

mod basic;
pub use self::{
    basic::*,
};

pub(super) fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<Player>()
        .register_component::<Loader>()
        .register_component::<Solid>()
        .register_component::<CameraTarget>()
}
