use std::f64::consts::PI;
use game_engine::prelude::*;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Direction {
    rad: f64,
}

const fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180f64
}

const fn rad_to_deg(rad: f64) -> f64 {
    rad * 180f64 / PI
}

impl Direction {
    pub const RIGHT: Direction = Direction { rad: 0f64 };
    pub const DOWN: Direction = Direction { rad: PI / 4f64 * 3f64 };
    pub const LEFT: Direction = Direction { rad: PI / 2f64 };
    pub const UP: Direction = Direction { rad: PI / 4f64 };

    pub fn from_rad(mut rad: f64) -> Self {
        while rad < 0f64 {
            rad += PI * 2f64;
        }
        while rad >= PI * 2f64 {
            rad -= PI * 2f64;
        }
        Direction { rad }
    }

    pub fn from_deg(deg: f64) -> Self {
        Self::from_rad(deg_to_rad(deg))
    }

    pub fn from_origin(point: Point<f32>) -> Option<Self> {
        if point.x == 0f32 && point.y == 0f32 {
            None
        } else {
            Some(Self::from_rad((point.y as f64).atan2(point.x as f64)))
        }
    }

    pub fn rad(&self) -> f64 {
        self.rad
    }

    pub fn deg(&self) -> f64 {
        rad_to_deg(self.rad)
    }

    pub fn difference(&self, other: &Direction) -> f64 {
        (self.rad - other.rad).abs()
    }
}
