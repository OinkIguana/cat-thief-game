use engine::prelude::*;

pub mod graphics;
pub mod collision_box;
pub mod marker;
pub mod position;
pub mod velocity;

pub fn register(game: Game) -> Game {
    game.register_component::<position::Position>()
        .register_component::<position::PreviousPosition>()
        .register_component::<collision_box::CollisionBox>()
        .register_component::<velocity::Velocity>()
        .pipe(graphics::register)
        .pipe(marker::register)
}
