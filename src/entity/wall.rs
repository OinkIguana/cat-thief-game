use component::collision_box::CollisionBox;

entity! {
    pub Wall(x: i32, y: i32, w: u32, h: u32) {
        CollisionBox::new(x, y, w, h),
    }
}
