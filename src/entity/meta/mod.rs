//! Meta entities, for things like the dialog system which needs to draw things but is really just
//! a system and some resources
use component::marker;

entity! {
    pub Dialog {
        marker::Dialog,
    }
}
