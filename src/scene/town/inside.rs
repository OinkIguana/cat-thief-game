use engine::prelude::*;

use constant::TILE_SIZE;
use system::{
    behaviors::{
        move_path::MoveByMovePath,
        doors::{EnterDoors, ExitDoors},
    },
    player::{
        dialog_control::DialogControl,
        movement::PlayerMovement,
    },
    basic::{
        apply_velocity::ApplyVelocity,
        camera_target::CameraTarget,
    },
    drawable::{
        sprite::MaintainSpriteDrawable,
        dialog::MaintainDialogDrawable,
    },
    animations::AnimateWalkCycle,
};
use entity::{
    meta::Dialog,
    door::Door,
};
use tile_grid::town_inside;

use super::outside::TOWN_OUTSIDE;

scene! {
    pub TOWN_INSIDE {
        entities: [
            Dialog,
            Door("house_4", TOWN_OUTSIDE, TILE_SIZE * 18, TILE_SIZE * 6, TILE_SIZE as u32, TILE_SIZE as u32 / 2, 0, TILE_SIZE),
            Door("shop", TOWN_OUTSIDE, TILE_SIZE * 6, TILE_SIZE * 11 + TILE_SIZE / 2, TILE_SIZE as u32, TILE_SIZE as u32, 0, -TILE_SIZE),
        ],
        systems: [
            (DialogControl::default(), "DialogControl", &[]),
            (ExitDoors::default(), "ExitDoors", &[]),
            (MoveByMovePath::default(), "MoveByMovePath", &["ExitDoors"]),
            (PlayerMovement::default(), "PlayerMovement", &["DialogControl", "MoveByMovePath"]),
            (ApplyVelocity::default(), "ApplyVelocity", &["PlayerMovement"]),
            (CameraTarget::default(), "CameraTarget", &["ApplyVelocity"]),
            (AnimateWalkCycle::default(), "AnimateWalkCycle", &["ApplyVelocity"]),
            (EnterDoors::default(), "EnterDoors", &["ApplyVelocity"]),
            (MaintainSpriteDrawable::default(), "MaintainSpriteDrawable", &["AnimateWalkCycle"]),
            (MaintainDialogDrawable::default(), "MaintainDialogDrawable", &["DialogControl"]),
        ]
    } => |builder| {
        {
            let mut layers = builder.get_resource_mut::<TileLayers>();
            layers.clear();
            layers.set(-5, town_inside::FLOOR.clone());
            layers.set(-4, town_inside::FLOOR_2.clone());
            layers.set(-3, town_inside::FURNITURE.clone());
            layers.set(1, town_inside::FURNITURE_FOREGROUND.clone());
            layers.set(2, town_inside::FURNITURE_FOREGROUND_2.clone());
        }
        builder.pipe(town_inside::collisions)
    }
}


