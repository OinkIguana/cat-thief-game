use engine::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct CollisionBox(pub Rect);

impl CollisionBox {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        CollisionBox(Rect::new(x, y, w, h))
    }
}
