use game_engine::prelude::*;

mod control;
mod debug;
mod dialog;

pub fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.plugin(control::process_control_events)
        .plugin(dialog::manage_dialog)
}
