use engine::prelude::*;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct AnimationSpeed(pub f32);

impl Default for AnimationSpeed {
    fn default() -> Self {
        AnimationSpeed(1f32)
    }
}

impl AnimationSpeed {
    pub fn new(speed: f32) -> Self {
        AnimationSpeed(speed)
    }
}
