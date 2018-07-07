use engine::prelude::*;

#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct Velocity(pub Point<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Point::new(x, y))
    }
}
