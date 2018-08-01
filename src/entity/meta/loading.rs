use game_engine::entity;
use crate::drawable::LoadingDrawable;
use crate::component::{
    marker::Loader,
    graphics::Visible,
};

entity! {
    pub Loading {
        LoadingDrawable::boxed(),
        Visible(true),
        Loader,
    }
}
