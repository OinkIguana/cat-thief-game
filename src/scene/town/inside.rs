use game_engine::scene;
use game_engine::prelude::*;

use crate::constant::TILE_SIZE;
use crate::entity::{
    meta::{Dialog, Loading},
    door::Door,
};
use crate::tile_grid::town_inside;
use crate::system::behaviors::doors::ExitDoors;

use super::outside::TOWN_OUTSIDE;

scene! {
    pub TOWN_INSIDE {
        entities: [
            Dialog,
            Loading,
            Door("house_4", TOWN_OUTSIDE, TILE_SIZE * 18, TILE_SIZE * 6, TILE_SIZE as u32, TILE_SIZE as u32 / 2, 0, TILE_SIZE),
            Door("shop", TOWN_OUTSIDE, TILE_SIZE * 6, TILE_SIZE * 11 + TILE_SIZE / 2, TILE_SIZE as u32, TILE_SIZE as u32, 0, -TILE_SIZE),
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
        builder
            .pipe(town_inside::collisions)
            .run_now(ExitDoors::default());
    }
}
