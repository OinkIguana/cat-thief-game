use system::player::movement::PlayerMovement;
use system::basic::apply_velocity::ApplyVelocity;
use system::basic::positioned_drawable::PositionedDrawable;
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
            (PositionedDrawable::default(), "PositionedDrawable", &["ApplyVelocity"]),
        ]
    }
}
