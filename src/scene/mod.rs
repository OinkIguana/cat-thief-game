use system::player::movement::PlayerMovement;
use system::basic::apply_velocity::ApplyVelocity;
use system::animations::AnimateWalkCycle;
use system::graphics::positioned_drawable::PositionedDrawable;
use system::graphics::sprite_drawable::SpriteDrawable;
use entity::player::Player;
use entity::wall::Wall;

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
