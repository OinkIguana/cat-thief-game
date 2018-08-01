use specs_derive::Component;
use game_engine::prelude::*;
use crate::component::behavior::MovePath;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct DoorID(pub &'static str);

#[derive(Component, Copy, Clone)]
pub struct TargetScene(pub &'static dyn Scene);

#[derive(Component, Copy, Clone, Debug)]
pub struct DoorExit(Point<f32>);

impl DoorExit {
    pub const fn new(x: i32, y: i32) -> Self {
        DoorExit(Point::new(x as f32, y as f32))
    }

    pub fn move_path(&self, position: Point<f32>) -> MovePath {
        MovePath::new(position + self.0)
    }
}
