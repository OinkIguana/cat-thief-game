use engine::Game;

mod sprite;
mod walk_cycle;
mod animation_speed;

pub use self::{
    sprite::{Sprite, SpriteFrame},
    animation_speed::AnimationSpeed,
    walk_cycle::WalkCycle,
};

pub(super) fn register(game: Game) -> Game {
    game.register_component::<Sprite>()
        .register_component::<SpriteFrame>()
        .register_component::<WalkCycle>()
        .register_component::<AnimationSpeed>()
}
