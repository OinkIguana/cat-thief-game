use model::direction::Direction;
use component::{
    marker,
    graphics::{SpriteFrame, DrawDepth, AnimationSpeed, WalkCycle},
    position::{Position, PreviousPosition},
    velocity::Velocity,
    collision_box::CollisionBox,
};
use drawable::SpriteDrawable;
use sprite::{MALE_WALKCYCLE, MALE_PANTS};

entity! {
    pub Player(x: i32, y: i32) {
        marker::Player,
        marker::Solid,
        marker::CameraTarget,
        Position::new(x, y),
        PreviousPosition::default(),
        Velocity::default(),
        CollisionBox::new(16, 32, 32, 32),
        SpriteFrame::new(18),
        DrawDepth::new(0),
        SpriteDrawable::boxed(vec![&MALE_WALKCYCLE, &MALE_PANTS]),
        AnimationSpeed::new(0.5),
        WalkCycle::new([
            (Direction::from_deg(270f64), 0..9),
            (Direction::from_deg(190f64), 9..18),
            (Direction::from_deg(90f64), 18..27),
            (Direction::from_deg(0f64), 27..36),
        ].iter().cloned())
    }
}
