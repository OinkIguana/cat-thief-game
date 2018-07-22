use engine::Game;

mod sprite;
mod draw_depth;
mod walk_cycle;
mod animation_speed;

pub use self::{
    sprite::{SpriteFrame, SpriteOrigin},
    draw_depth::DrawDepth,
    animation_speed::AnimationSpeed,
    walk_cycle::WalkCycle,
};

pub(super) fn register(game: Game) -> Game {
    game.register_component::<SpriteFrame>()
        .register_component::<SpriteOrigin>()
        .register_component::<DrawDepth>()
        .register_component::<WalkCycle>()
        .register_component::<AnimationSpeed>()
}
