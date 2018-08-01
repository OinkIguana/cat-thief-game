use game_engine::entity;
use crate::drawable::DialogDrawable;

entity! {
    pub Dialog {
        DialogDrawable::boxed(),
    }
}
