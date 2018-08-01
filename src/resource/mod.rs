use engine::prelude::*;

pub mod constant;
pub mod control_events;
pub mod control_scheme;
pub mod dialog_messages;
pub mod door_transition;
pub mod state;

pub fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(constant::BaseMovementSpeed::default())
        .add_resource(constant::DialogSpeed::default())
        .add_resource(control_events::ControlEvents::default())
        .add_resource(control_events::ControlState::default())
        .add_resource(control_scheme::ControlScheme::default())
        .add_resource(dialog_messages::DialogMessages::default())
        .add_resource(dialog_messages::DialogProgress::default())
        .add_resource(door_transition::DoorTransition::new("shop"))
        .add_resource(state::State::default())
}
