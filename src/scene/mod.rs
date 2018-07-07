use system::player::movement::PlayerMovement;
use system::basic::apply_velocity::ApplyVelocity;
use system::basic::positioned_drawable::PositionedDrawable;
use entity::player::Player;

scene! {
    pub START {
        entities: [
            Player,
        ],
        systems: [
            (PlayerMovement::default(), "PlayerMovement", &[]),
            (ApplyVelocity::default(), "ApplyVelocity", &["PlayerMovement"]),
            (PositionedDrawable::default(), "PositionedDrawable", &["ApplyVelocity"]),
        ]
    }
}
