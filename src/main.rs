#![feature(macro_at_most_once_rep, range_contains, const_fn)]
#![deny(bare_trait_objects)]
#![allow(dead_code)] // while still in early development, there's a lot of stuff unused.

#[macro_use] extern crate game_engine as engine;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate specs;
#[macro_use] extern crate specs_derive;
#[macro_use] extern crate lazy_static;

mod component;
mod constant;
mod dialog;
mod drawable;
mod entity;
mod font;
mod image;
mod model;
mod plugin;
mod resource;
mod scene;
mod sprite;
mod system;
mod tile_grid;
mod tile_set;

use engine::prelude::*;

fn main() -> engine::Result<()> {
    Game::new()
        .pipe(component::register)
        .pipe(resource::register)
        .pipe(plugin::register)
        .start(scene::START)
}
