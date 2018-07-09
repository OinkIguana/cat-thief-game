//! Uses the [`Position`] component to move the [`Drawable`]
use engine::prelude::*;
use component::position::Position;

#[derive(Default, Debug)]
pub struct PositionedDrawable;

system! {
    impl PositionedDrawable {
        fn run(
            &mut self,
            position: &Component<Position>,
            drawable: &mut Component<Drawable>,
        ) {
            for (position, mut drawable) in (&position, &mut drawable).join() {
                drawable.set_position(position.rounded());
            }
        }
    }
}
