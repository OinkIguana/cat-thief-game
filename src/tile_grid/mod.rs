use lazy_static::lazy_static;
use game_engine::prelude::*;
use crate::entity::wall::Wall;
use crate::tile_set;

include!(concat!(env!("OUT_DIR"), "/tile_grids.rs"));
