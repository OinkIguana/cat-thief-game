use engine::prelude::*;

pub mod behavior;
pub mod graphics;
pub mod marker;
pub mod collision_box;
pub mod door;
pub mod position;
pub mod velocity;

pub fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<position::Position>()
        .register_component::<position::PreviousPosition>()
        .register_component::<collision_box::CollisionBox>()
        .register_component::<velocity::Velocity>()
        .register_component::<door::DoorID>()
        .register_component::<door::TargetScene>()
        .register_component::<door::DoorExit>()
        .pipe(graphics::register)
        .pipe(marker::register)
        .pipe(behavior::register)
}
