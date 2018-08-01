use game_engine::system;
use crate::component::{
    marker::Loader,
    graphics::Visible,
};

#[derive(Default, Debug)]
pub struct ShowLoader;

system! {
    impl ShowLoader {
        fn run(
            &mut self,
            loader: &Component<Loader>,
            visible: &mut Component<Visible>,
        ) {
            for (_, visible) in (&loader, &mut visible).join() {
                visible.0 = true;
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct HideLoader;

system! {
    impl HideLoader {
        fn run(
            &mut self,
            loader: &Component<Loader>,
            visible: &mut Component<Visible>,
        ) {
            for (_, visible) in (&loader, &mut visible).join() {
                visible.0 = false;
            }
        }
    }
}
