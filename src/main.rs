#![feature(macro_at_most_once_rep)]
#![deny(bare_trait_objects)]

#[macro_use] extern crate game_engine as engine;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate specs;
#[macro_use] extern crate specs_derive;

mod component;
mod entity;
mod image;
mod model;
mod plugin;
mod resource;
mod scene;
mod sprite;
mod system;

use engine::prelude::*;

fn main() -> engine::Result<()> {
    Game::new()
        .pipe(component::register)
        .pipe(resource::register)
        .pipe(plugin::register)
        .start(scene::START)
}
