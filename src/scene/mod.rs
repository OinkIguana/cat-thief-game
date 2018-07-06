use system::player::movement::PlayerMovement;
use system::basic::positioned_drawable::PositionedDrawable;
use entity::player::Player;

scene! {
    pub START {
        entities: [
            Player,
        ],
        systems: [
            (PlayerMovement::default(), "PlayerMovement", &[]),
            (PositionedDrawable::default(), "PositionedDrawable", &["PlayerMovement"]),
        ]
    }
}
