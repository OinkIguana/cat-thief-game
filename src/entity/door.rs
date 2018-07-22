use engine::prelude::*;
use component::{
    position::Position,
    collision_box::CollisionBox,
    door::{DoorID, TargetScene, DoorExit},
};

entity! {
    pub Door(
        name: &'static str, 
        scene: &'static dyn Scene,
        x: i32, 
        y: i32, 
        width: u32, 
        height: u32, 
        exit_x: i32,
        exit_y: i32,
    ) {
        Position::new(x, y),
        CollisionBox::new(0, 0, width, height),
        TargetScene(scene),
        DoorID(name),
        DoorExit::new(exit_x, exit_y),
    }
}
