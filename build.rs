extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::env;
use std::path::PathBuf;
use std::fs::{self, File, ReadDir};
use std::io::Write;
use std::ffi::OsStr;

use toml::{from_str, Value};

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
}

fn write_images<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            writeln!(file, "pub mod {} {{", path.file_name().unwrap().to_str().unwrap().to_owned().to_lowercase()).unwrap();
            writeln!(file, "use super::Image;").unwrap();
            let sub_paths = fs::read_dir(path).unwrap();
            write_images(file, sub_paths);
            writeln!(file, "}}").unwrap();
        } else if path.extension() != Some(&OsStr::new("rs")) {
            let name = path.file_stem().unwrap();
            let const_name = name.to_str().unwrap().to_owned().to_uppercase();
            writeln!(file, "pub const {}: Image = Image::new({:?});", const_name, path.to_str().unwrap()).unwrap();
        }
    }
}

#[derive(Deserialize)]
struct SpriteSpec {
    image: String,
    frames: Vec<[u32; 4]>,
}

fn write_sprites<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            writeln!(file, "pub mod {} {{", path.file_name().unwrap().to_str().unwrap().to_owned().to_lowercase()).unwrap();
            writeln!(file, "use super::{{image, Sprite, Rect}};").unwrap();
            let sub_paths = fs::read_dir(path).unwrap();
            write_sprites(file, sub_paths);
            writeln!(file, "}}").unwrap();
        } else if path.extension() == Some(&OsStr::new("toml")) {
            let name = path.file_stem().unwrap();
            let const_name = name.to_str().unwrap().to_owned().to_uppercase();
            let toml_str = fs::read_to_string(&path).unwrap();
            let sprite: SpriteSpec = from_str(&toml_str).unwrap();
            writeln!(file, "pub const {}: Sprite = Sprite::new(image::{}, &[", const_name, sprite.image).unwrap();
            for [x, y, w, h] in sprite.frames {
                writeln!(file, "Rect::new({}, {}, {}, {}),", x, y, w, h).unwrap();
            }
            writeln!(file, "]);").unwrap();
        }
    }
}

fn write_fonts<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if !path.is_dir() && path.extension() == Some(&OsStr::new("toml")) {
            let name = path.file_stem().unwrap();
            writeln!(file, "pub mod {} {{", name.to_str().unwrap().to_owned()).unwrap();
            writeln!(file, "use super::Font;").unwrap();
            let toml_str = fs::read_to_string(&path).unwrap();
            let value = toml_str.parse::<Value>().unwrap();
            let styles = value["styles"].as_array().unwrap();
            for style in styles {
                let filename = style["file"].as_str().unwrap();
                let stylename = style["name"].as_str().unwrap();
                let sizes = style["sizes"].as_array().unwrap();
                for size in sizes {
                    let size = size.as_integer().unwrap();
                    let const_name = format!("{}_{}", stylename.to_uppercase(), size);
                    let mut file_path = PathBuf::from("font/ttf");
                    file_path.push(filename);
                    writeln!(file, "pub const {}: Font = Font::new({:?}, {});", const_name, file_path, size).unwrap();
                }
            }
            writeln!(file, "}}").unwrap();
        }
    }
}

#[derive(Deserialize)]
struct Dimen {
    width: u32,
    height: u32,
}

impl ::std::fmt::Display for Dimen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Dimen::new({}, {})", self.width, self.height)
    }
}

#[derive(Deserialize)]
struct Point {
    x: u32,
    y: u32,
}

impl ::std::fmt::Display for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Point::new({}, {})", self.x, self.y)
    }
}

#[derive(Deserialize)]
struct TileSetSpec {
    image: String,
    count: u32,
    per_row: u32,
    size: Dimen,
    origin: Point,
    spacing: Dimen,
}

fn write_tile_sets<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            writeln!(file, "pub mod {} {{", path.file_name().unwrap().to_str().unwrap().to_owned().to_lowercase()).unwrap();
            writeln!(file, "use super::{{image, TileSet, Point, Dimen}};").unwrap();
            let sub_paths = fs::read_dir(path).unwrap();
            write_tile_sets(file, sub_paths);
            writeln!(file, "}}").unwrap();
        } else if path.extension() == Some(&OsStr::new("toml")) {
            let name = path.file_stem().unwrap();
            let const_name = name.to_str().unwrap().to_owned().to_uppercase();
            let toml_str = fs::read_to_string(&path).unwrap();
            let tile_set: TileSetSpec = from_str(&toml_str).unwrap();
            writeln!(
                file, 
                "pub const {}: TileSet = TileSet::new(&image::{}, {}, {}, {}, {}, {});", 
                const_name, 
                tile_set.image,
                tile_set.count,
                tile_set.per_row,
                tile_set.size,
                tile_set.origin,
                tile_set.spacing,
            ).unwrap();
        }
    }
}

