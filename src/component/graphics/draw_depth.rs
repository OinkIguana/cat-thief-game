use specs_derive::Component;
use game_engine::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct DrawDepth(pub i32);

impl DrawDepth {
    pub fn new(depth: i32) -> Self {
        DrawDepth(depth)
    }
}
