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
use tile_grid::town;
use resource::dialog_messages::DialogMessages;
use dialog;
use super::inside::TOWN_INSIDE;

scene! {
    pub TOWN_OUTSIDE {
        entities: [
            Dialog,
            Door("house_4", TOWN_INSIDE, TILE_SIZE * 30, TILE_SIZE * 12, TILE_SIZE as u32, TILE_SIZE as u32, 0, -TILE_SIZE),
            Door("shop", TOWN_INSIDE, TILE_SIZE * 14, TILE_SIZE * 2, TILE_SIZE as u32, TILE_SIZE as u32 / 2, 0, TILE_SIZE),
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
            layers.set(-5, town::WATER.clone());
            layers.set(-4, town::GROUND.clone());
            layers.set(-3, town::WALLS.clone());
            layers.set(-2, town::OBSTACLES.clone());
            layers.set(-1, town::DOORS.clone());
            layers.set(1, town::ROOFS.clone());
        }
        builder.get_resource_mut::<DialogMessages>().start(dialog::opening);
        builder.pipe(town::collisions)
    }
}

