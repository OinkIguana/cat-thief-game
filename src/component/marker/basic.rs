use specs_derive::Component;
use game_engine::prelude::*;

/// Solid entities' collision boxes cannot intersect.
#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Solid;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct CameraTarget;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Loader;

#[derive(Component, Copy, Clone, Default, Debug)]
#[storage(NullStorage)]
pub struct Pickup;
