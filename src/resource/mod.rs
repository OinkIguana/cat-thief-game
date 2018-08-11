use game_engine::prelude::*;

pub mod constant;
pub mod control;
pub mod cutscene;
pub mod dialog;
pub mod door_transition;
pub mod state;

pub fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(constant::BaseMovementSpeed::default())
        .add_resource(door_transition::DoorTransition::new("shop"))
        .add_resource(state::State::default())
        .pipe(control::register)
        .pipe(dialog::register)
        .pipe(cutscene::register)
}
