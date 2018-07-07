use engine::Game;

mod sprite;
pub use self::sprite::{Sprite, SpriteFrame};
mod walk_cycle;
pub use self::walk_cycle::WalkCycle;
mod animation_speed;
pub use self::animation_speed::AnimationSpeed;

pub(super) fn register(game: Game) -> Game {
    game.register_component::<Sprite>()
        .register_component::<SpriteFrame>()
        .register_component::<WalkCycle>()
        .register_component::<AnimationSpeed>()
}
