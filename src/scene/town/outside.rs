use game_engine::scene;
use game_engine::prelude::*;

use crate::constant::TILE_SIZE;
use crate::entity::{
    meta::{Dialog, Loading},
    door::Door,
    state_pickup::StatePickup,
};
use crate::tile_grid::town;
use crate::resource::{
    dialog_messages::DialogMessages,
    state::{State, MainState},
};
use crate::dialog;
use crate::system::behaviors::doors::ExitDoors;
use super::inside::TOWN_INSIDE;

scene! {
    pub TOWN_OUTSIDE {
        entities: [
            Dialog,
            Loading,
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
        if builder.get_resource::<State>().main == MainState::Start {
            builder.get_resource_mut::<DialogMessages>().start(dialog::opening);
            builder.get_resource_mut::<State>().main = MainState::RunToTheAlley;
        }
        if builder.get_resource::<State>().main == MainState::RunToTheAlley {
            builder.add_entity(StatePickup(TILE_SIZE * 24, TILE_SIZE * 7, TILE_SIZE as u32, TILE_SIZE as u32, MainState::End));
        }
        builder
            .pipe(town::collisions)
            .run_now(ExitDoors::default());
    }
}
