use engine::prelude::*;

pub mod position;
pub mod marker;

pub fn register(game: Game) -> Game {
    game.register_component::<position::Position>()
        .pipe(marker::register)
}
