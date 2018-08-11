use specs_derive::Component;
use game_engine::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct MysteryMan;

pub(super) fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<Player>()
        .register_component::<MysteryMan>()
}
