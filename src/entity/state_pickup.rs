use resource::state::MainState;
use component::{
    collision_box::CollisionBox,
    state_target::StateTarget,
    marker,
};

entity! {
    pub StatePickup(x: i32, y: i32, w: u32, h: u32, state: MainState) {
        marker::Pickup,
        CollisionBox::new(x, y, w, h),
        StateTarget::new(state),
    }
}
