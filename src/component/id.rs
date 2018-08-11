use specs_derive::Component;
use game_engine::prelude::*;

/// An Id provides runtime identification for individual entities, different from the `marker::ids`
/// components which provide compile time identification.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Id {
    Player,
    MysteryMan,
}
