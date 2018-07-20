use engine::prelude::*;

use constant::TILE_SIZE;
use system::{
    player::movement::PlayerMovement,
    basic::{
        apply_velocity::ApplyVelocity,
        camera_target::CameraTarget,
    },
    drawable::{
        sprite::MaintainSpriteDrawable,
    },
    animations::AnimateWalkCycle,
};
use entity::{
    meta::Dialog,
    player::Player,
};
use tile_grid::town;

scene! {
    pub START {
        entities: [
            Dialog,
            Player(TILE_SIZE * 20 + TILE_SIZE / 2, 10 * TILE_SIZE),
        ],
        systems: [
            (PlayerMovement::default(), "PlayerMovement", &[]),
            (ApplyVelocity::default(), "ApplyVelocity", &["PlayerMovement"]),
            (CameraTarget::default(), "CameraTarget", &["ApplyVelocity"]),
            (AnimateWalkCycle::default(), "AnimateWalkCycle", &["ApplyVelocity"]),
            (MaintainSpriteDrawable::default(), "MaintainSpriteDrawable", &["AnimateWalkCycle"]),
        ]
    } => |builder| {
        {
            let mut layers = builder.get_resource_mut::<TileLayers>();
            layers.set(-5, town::WATER.clone());
            layers.set(-4, town::GROUND.clone());
            layers.set(-3, town::WALLS.clone());
            layers.set(-2, town::OBSTACLES.clone());
            layers.set(-1, town::DOORS.clone());
            layers.set(1, town::ROOFS.clone());
        }
        builder.pipe(town::collisions)
    }
}
