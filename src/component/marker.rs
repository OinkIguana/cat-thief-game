use engine::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Player;

pub fn register(game: Game) -> Game {
    game.register_component::<Player>()
}
