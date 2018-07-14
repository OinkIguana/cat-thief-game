use engine::prelude::*;
use component::{
    collision_box::CollisionBox,
    marker,
};

entity! {
    pub Wall(x: i32, y: i32, w: u32, h: u32) {
        marker::Solid,
        CollisionBox::new(x, y, w, h),
        Drawable::new().rect(Rect::new(x, y, w, h)).color(Color::WHITE).build()
    }
}
