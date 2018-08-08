use game_engine::{system, prelude::*};

use crate::component::{
    marker,
    position::Position,
};

#[derive(Default, Debug)]
pub struct CameraTarget;

system! {
    impl CameraTarget {
        fn run(
            &mut self,
            position: &Component<Position>,
            camera_target: &Component<marker::CameraTarget>,
            camera: &mut Resource<Camera>,
            current_scene: &Resource<CurrentScene>,
        ) {
            for (position, _) in (&position, &camera_target).join() {
                camera.center_on(position.rounded()).keep_within(current_scene.current().bounds());
            }
        }
    }
}
