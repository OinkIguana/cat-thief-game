use game_engine::prelude::*;

mod control;
mod debug;

pub fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.plugin(control::process_control_events)
}
