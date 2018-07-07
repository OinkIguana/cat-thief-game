use engine::prelude as engine;
use specs::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Sprite(pub &'static engine::Sprite);

impl Sprite {
    pub fn new(sprite: &'static engine::Sprite) -> Self {
        Sprite(sprite)
    }
}

#[derive(Component, Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct SpriteFrame(pub f32);

impl SpriteFrame {
    pub fn new(frame: i32) -> Self {
        SpriteFrame(frame as f32)
    }

    pub fn current(&self) -> usize {
        self.0 as usize
    }
}
