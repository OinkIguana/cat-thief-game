use game_engine::prelude::*;

pub mod behavior;
pub mod graphics;
pub mod marker;
pub mod collision_box;
pub mod door;
pub mod id;
pub mod position;
pub mod state_target;
pub mod velocity;

pub fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<position::Position>()
        .register_component::<position::PreviousPosition>()
        .register_component::<collision_box::CollisionBox>()
        .register_component::<velocity::Velocity>()
        .register_component::<id::Id>()
        .register_component::<door::DoorID>()
        .register_component::<door::TargetScene>()
        .register_component::<door::DoorExit>()
        .register_component::<state_target::StateTarget>()
        .pipe(graphics::register)
        .pipe(marker::register)
        .pipe(behavior::register)
}
