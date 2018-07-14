use engine::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Player;

/// Solid entities collision boxes cannot intersect.
#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Solid;

pub(super) fn register(game: Game) -> Game {
    game.register_component::<Player>()
        .register_component::<Solid>()
}


