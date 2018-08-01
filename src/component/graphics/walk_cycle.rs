use specs_derive::Component;
use std::ops::Range;
use game_engine::prelude::*;

use crate::model::direction::Direction;

// NOTE: I feel like `HashMap` has much overhead for something so simple as this. Might be worth
// looking into `Vec<(Direction, &[Rect])>` implementation.

#[derive(Component, Clone, Debug)]
pub struct WalkCycle {
    pub directions: Vec<(Direction, Range<usize>)>,
}

impl WalkCycle {
    pub fn new(directions: impl Iterator<Item = (Direction, Range<usize>)>) -> Self {
        WalkCycle { directions: directions.collect() }
    }

    pub fn frames(&self, direction: Direction) -> Option<Range<usize>> {
        self.directions
            .iter()
            .fold(None, |best: Option<(Direction, Range<usize>)>, next| {
                if let Some(best) = best {
                    if direction.difference(&next.0) < direction.difference(&best.0) {
                        Some(next.clone())
                    } else {
                        Some(best)
                    }
                } else {
                    Some(next.clone())
                }
            })
            .map(|(_, rects)| rects)
    }
}
