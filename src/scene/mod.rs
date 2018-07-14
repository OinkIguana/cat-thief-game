use engine::prelude::*;

use system::{
    player::movement::PlayerMovement,
    basic::apply_velocity::ApplyVelocity,
    animations::AnimateWalkCycle,
    graphics::{
        positioned_drawable::PositionedDrawable,
        sprite_drawable::SpriteDrawable,
    },
};
use entity::player::Player;
use tile_grid::town;

scene! {
    pub START {
        entities: [
            Player(656, 320),
        ],
        systems: [
            (PlayerMovement::default(), "PlayerMovement", &[]),
            (ApplyVelocity::default(), "ApplyVelocity", &["PlayerMovement"]),
            (AnimateWalkCycle::default(), "AnimateWalkCycle", &["ApplyVelocity"]),
            (PositionedDrawable::default(), "PositionedDrawable", &["ApplyVelocity"]),
            (SpriteDrawable::default(), "SpriteDrawable", &["AnimateWalkCycle"]),
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
