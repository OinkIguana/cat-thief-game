use game_engine::Game;

mod dialog_events;
mod dialog_messages;
mod dialog_selection;
mod dialog_progress;
mod dialog_speed;

pub use self::{
    dialog_events::*,
    dialog_messages::*,
    dialog_selection::*,
    dialog_progress::*,
    dialog_speed::*,
};

pub fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(DialogEvents::default())
        .add_resource(DialogSpeed::default())
        .add_resource(DialogMessages::default())
        .add_resource(DialogSelection::default())
        .add_resource(DialogProgress::default())
}
