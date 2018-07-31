use engine::Game;

mod animation_speed;
mod draw_depth;
mod sprite;
mod visible;
mod walk_cycle;

pub use self::{
    sprite::{SpriteFrame, SpriteOrigin},
    draw_depth::DrawDepth,
    animation_speed::AnimationSpeed,
    walk_cycle::WalkCycle,
    visible::Visible,
};

pub(super) fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<SpriteFrame>()
        .register_component::<SpriteOrigin>()
        .register_component::<DrawDepth>()
        .register_component::<WalkCycle>()
        .register_component::<Visible>()
        .register_component::<AnimationSpeed>()
}
