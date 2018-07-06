use engine::prelude::*;

pub mod constant;
pub mod control_events;
pub mod control_scheme;

pub fn register(game: Game) -> Game {
    game.add_resource(constant::BaseMovementSpeed(4))
        .add_resource(control_events::ControlEvents::default())
        .add_resource(control_events::ControlState::default())
        .add_resource(control_scheme::ControlScheme::default())
}
