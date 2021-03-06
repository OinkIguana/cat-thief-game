use specs_derive::Component;
use game_engine::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Clone, Default, Debug)]
pub struct MovePath {
    target: Option<Point<f32>>,
    points: VecDeque<Point<f32>>,
}

impl MovePath {
    pub fn from(points: &[Point<f32>]) -> Self {
        if points.is_empty() {
            Self::default()
        } else {
            Self {
                points: VecDeque::from(points[1..].to_vec()),
                target: points.get(0).cloned(),
            }
        }
    }

    pub fn new(point: Point<f32>) -> Self {
        let mut path = MovePath::default();
        path.add(point);
        path
    }

    pub fn is_empty(&self) -> bool {
        self.target.is_none() && self.points.is_empty()
    }

    pub fn add(&mut self, point: Point<f32>) {
        if self.target.is_none() {
            self.target = Some(point);
        } else {
            self.points.push_back(point);
        }
    }

    pub fn target(&self) -> Option<Point<f32>> {
        self.target
    }

    pub fn arrive(&mut self) {
        self.target = self.points.pop_front();
    }

    pub fn destination(&self) -> Option<Point<f32>> {
        self.points.back().cloned()
    }
}
