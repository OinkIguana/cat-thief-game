use engine::prelude::*;
use component::marker;
use component::position::Position;

entity! {
    pub Player {
        marker::Player::default(),
        Position::new(0, 0),
        Drawable::new().rect(Rect::new(0, 0, 32, 32)).color(Color::RED).build(),
    }
}
