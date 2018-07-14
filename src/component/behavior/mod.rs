use engine::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Default, Debug)]
pub struct MovePath(VecDeque<Point>);

impl MovePath {
    pub fn new(point: Point) -> Self {
        let mut path = MovePath::default();
        path.add(point);
        path
    }

    pub fn next(&self) -> Point {
        self.0[0]
    }

    pub fn add(&mut self, point: Point) {
        self.0.push_back(point);
    }

    pub fn arrive(&mut self) {
        self.0.pop_front();
    }
}
