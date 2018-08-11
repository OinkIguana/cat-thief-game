use game_engine::prelude::*;

pub mod constant;
pub mod control;
pub mod dialog;
pub mod door_transition;
pub mod state;

pub fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(constant::BaseMovementSpeed::default())
        .pipe(control::register)
        .pipe(dialog::register)
        .add_resource(door_transition::DoorTransition::new("shop"))
        .add_resource(state::State::default())
}
