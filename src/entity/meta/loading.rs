use drawable::LoadingDrawable;
use component::{
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
