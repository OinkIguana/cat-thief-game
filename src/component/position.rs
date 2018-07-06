use engine::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Position(pub Point);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position(Point::new(x, y))
    }
}
