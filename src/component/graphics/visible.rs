use engine::prelude::*;

#[derive(Component, Eq, PartialEq, Debug)]
pub struct Visible(pub bool);

impl Default for Visible {
    fn default() -> Self {
        Visible(true)
    }
}
