use specs_derive::Component;
use game_engine::prelude::*;

#[derive(Component, Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct SpriteFrame(pub f32);

impl SpriteFrame {
    pub const fn new(frame: i32) -> Self {
        SpriteFrame(frame as f32)
    }

    pub fn current(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct SpriteOrigin(pub Point);

impl SpriteOrigin {
    pub const fn new(x: i32, y: i32) -> Self {
        SpriteOrigin(Point::new(x, y))
    }
}
