use engine::prelude::*;

mod control;

pub fn register(game: Game) -> Game {
    game.plugin(control::process_control_events)
}
