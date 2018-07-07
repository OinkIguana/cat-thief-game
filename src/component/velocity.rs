use engine::prelude::*;
use model::direction::Direction;

#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct Velocity(pub Point<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Point::new(x, y))
    }

    pub fn magnitude(&self) -> f32 {
        (self.0.x.powi(2) + self.0.y.powi(2)).sqrt()
    }

    pub fn direction(&self) -> Option<Direction> {
        Direction::from_origin(self.0)
    }
}
