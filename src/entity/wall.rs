use component::{
    collision_box::CollisionBox,
    marker,
};

entity! {
    pub Wall(x: i32, y: i32, w: u32, h: u32) {
        marker::Solid,
        CollisionBox::new(x, y, w, h),
    }
}
