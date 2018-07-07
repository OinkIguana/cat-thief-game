use engine::prelude::*;
use component::marker;
use component::position::Position;
use component::velocity::Velocity;
use component::collision_box::CollisionBox;

entity! {
    pub Player(x: i32, y: i32) {
        marker::Player::default(),
        Position::new(x, y),
        Velocity::default(),
        CollisionBox::new(0, 0, 32, 32),
        Drawable::new().rect(Rect::new(0, 0, 32, 32)).color(Color::RED).build(),
    }
}
