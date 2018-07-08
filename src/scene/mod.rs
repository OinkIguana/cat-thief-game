use system::{
    player::movement::PlayerMovement,
    basic::apply_velocity::ApplyVelocity,
    animations::AnimateWalkCycle,
    graphics::{
        positioned_drawable::PositionedDrawable,
        sprite_drawable::SpriteDrawable,
    },
};
use entity::{
    player::Player,
    wall::Wall,
};

scene! {
    pub START {
        entities: [
            Player(128, 128),
            Wall(0, 0, 1024, 32),
        ],
        systems: [
            (PlayerMovement::default(), "PlayerMovement", &[]),
            (ApplyVelocity::default(), "ApplyVelocity", &["PlayerMovement"]),
            (AnimateWalkCycle::default(), "AnimateWalkCycle", &["ApplyVelocity"]),
            (PositionedDrawable::default(), "PositionedDrawable", &["ApplyVelocity"]),
            (SpriteDrawable::default(), "SpriteDrawable", &["AnimateWalkCycle"]),
        ]
    }
}
