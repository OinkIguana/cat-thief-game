#![feature(generators, generator_trait)]

extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;

use std::{
    env,
    path::PathBuf,
    fs::{self, File},
};

mod schema;
mod image;
mod sprite;
mod font;
mod tile_set;
mod tile_grid;
mod dialog;

use self::{
    image::*,
    sprite::*,
    font::*,
    tile_set::*,
    tile_grid::*,
    dialog::*,
};

// Generates the images, sprites, and fonts modules
fn main() {
    let mut resources_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    resources_dir.push("src");
    let dest_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let images_dir = resources_dir.join("image");
    let images_out_path = dest_path.join("images.rs");
    let mut images_out_file = File::create(images_out_path).unwrap();
    write_images(&mut images_out_file, fs::read_dir(images_dir).unwrap());

    let sprites_dir = resources_dir.join("sprite");
    let sprites_out_path = dest_path.join("sprites.rs");
    let mut sprites_out_file = File::create(sprites_out_path).unwrap();
    write_sprites(&mut sprites_out_file, fs::read_dir(sprites_dir).unwrap());

    let fonts_dir = resources_dir.join("font");
    let fonts_out_path = dest_path.join("fonts.rs");
    let mut fonts_out_file = File::create(fonts_out_path).unwrap();
    write_fonts(&mut fonts_out_file, fs::read_dir(fonts_dir).unwrap());

    let tile_sets_dir = resources_dir.join("tile_set");
    let tile_sets_out_path = dest_path.join("tile_sets.rs");
    let mut tile_sets_out_file = File::create(tile_sets_out_path).unwrap();
    write_tile_sets(&mut tile_sets_out_file, fs::read_dir(tile_sets_dir).unwrap());

    let tile_grids_dir = resources_dir.join("tile_grid");
    let tile_grids_out_path = dest_path.join("tile_grids.rs");
    let mut tile_grids_out_file = File::create(tile_grids_out_path).unwrap();
    write_tile_grids(&mut tile_grids_out_file, fs::read_dir(tile_grids_dir).unwrap());

    let dialogs_dir = resources_dir.join("dialog");
    let dialogs_out_path = dest_path.join("dialogs.rs");
    let mut dialogs_out_file = File::create(dialogs_out_path).unwrap();
    write_dialogs(&mut dialogs_out_file, fs::read_dir(dialogs_dir).unwrap());
}
