use specs_derive::Component;
use game_engine::prelude::*;

#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct Position(pub Point<f32>);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position(Point::new(x as f32, y as f32))
    }

    pub fn rounded(&self) -> Point {
        Point::new(self.0.x as i32, self.0.y as i32)
    }
}

#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct PreviousPosition(pub Point<f32>);

impl PreviousPosition {
    pub fn new(x: i32, y: i32) -> Self {
        PreviousPosition(Point::new(x as f32, y as f32))
    }

    pub fn rounded(&self) -> Point {
        Point::new(self.0.x as i32, self.0.y as i32)
    }
}
