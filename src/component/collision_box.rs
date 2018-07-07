use engine::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct CollisionBox(pub Rect);

impl CollisionBox {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        CollisionBox(Rect::new(x, y, w, h))
    }

    pub fn at(&self, position: Point<f32>) -> Rect {
        Rect::new(self.0.x + position.x as i32, self.0.y + position.y as i32, self.0.width, self.0.height)
    }
}
