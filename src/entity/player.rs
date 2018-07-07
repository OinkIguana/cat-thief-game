use engine::prelude::*;
use component::marker;
use component::graphics::{Sprite, SpriteFrame, AnimationSpeed, WalkCycle};
use component::position::{Position, PreviousPosition};
use component::velocity::Velocity;
use component::collision_box::CollisionBox;
use sprite::MALE_WALKCYCLE;
use model::direction::Direction;

entity! {
    pub Player(x: i32, y: i32) {
        marker::Player::default(),
        Position::new(x, y),
        PreviousPosition::default(),
        Velocity::default(),
        CollisionBox::new(0, 0, 32, 32),
        Sprite::new(&MALE_WALKCYCLE),
        SpriteFrame::default(),
        Drawable::default(),
        AnimationSpeed::new(0.5),
        WalkCycle::new([
            (Direction::from_deg(270f64), 0..9),
            (Direction::from_deg(190f64), 9..18),
            (Direction::from_deg(90f64), 18..27),
            (Direction::from_deg(0f64), 27..36),
        ].iter().cloned())
    }
}
