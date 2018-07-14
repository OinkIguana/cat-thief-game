use engine::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Player;

/// Solid entities collision boxes cannot intersect.
#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Solid;
