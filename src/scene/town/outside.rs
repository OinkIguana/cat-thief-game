use engine::prelude::*;

use constant::TILE_SIZE;
use entity::{
    meta::Dialog,
    door::Door,
};
use tile_grid::town;
use resource::dialog_messages::DialogMessages;
use dialog;
use system::behaviors::doors::ExitDoors;
use super::inside::TOWN_INSIDE;

scene! {
    pub TOWN_OUTSIDE {
        entities: [
            Dialog,
            Door("house_4", TOWN_INSIDE, TILE_SIZE * 30, TILE_SIZE * 12, TILE_SIZE as u32, TILE_SIZE as u32, 0, -TILE_SIZE),
            Door("shop", TOWN_INSIDE, TILE_SIZE * 14, TILE_SIZE * 2, TILE_SIZE as u32, TILE_SIZE as u32 / 2, 0, TILE_SIZE),
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
        builder
            .pipe(town::collisions)
            .run_now(ExitDoors::default());
    }
}
