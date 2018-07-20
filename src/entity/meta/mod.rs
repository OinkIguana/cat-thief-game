//! Meta entities, for things like the dialog system which needs to draw things but is really just
//! a system and some resources
use drawable::DialogDrawable;

entity! {
    pub Dialog {
        DialogDrawable::boxed(),
    }
}
