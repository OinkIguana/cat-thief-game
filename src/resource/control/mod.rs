use game_engine::Game;

mod control_events;
mod control_scheme;

pub use self::{
    control_events::*,
    control_scheme::*,
};

pub(super) fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(ControlEvents::default())
        .add_resource(ControlState::default())
        .add_resource(ControlScheme::default())
}
