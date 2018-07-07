extern crate toml;

use std::env;
use std::path::PathBuf;
use std::fs::{self, File, ReadDir};
use std::io::Write;
use std::ffi::OsStr;

use toml::Value;

// Generates the images, sprites, and fonts modules
fn main() {
    let mut resources_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    resources_dir.push("src");
    let images_dir = resources_dir.join("image");
    let sprites_dir = resources_dir.join("sprite");
    let fonts_dir = resources_dir.join("font");

    let dest_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let images_out_path = dest_path.join("images.rs");
    let sprites_out_path = dest_path.join("sprites.rs");
    let fonts_out_path = dest_path.join("fonts.rs");
    let mut images_out_file = File::create(images_out_path).unwrap();
    write_images(&mut images_out_file, fs::read_dir(images_dir).unwrap());
    let mut sprites_out_file = File::create(sprites_out_path).unwrap();
    write_sprites(&mut sprites_out_file, fs::read_dir(sprites_dir).unwrap());
    let mut fonts_out_file = File::create(fonts_out_path).unwrap();
    write_fonts(&mut fonts_out_file, fs::read_dir(fonts_dir).unwrap());
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
            let value = toml_str.parse::<Value>().unwrap();
            let image_path = value["image"].as_str().unwrap();
            let rects = value["frames"].as_array().unwrap();
            writeln!(file, "pub const {}: Sprite = Sprite::new(image::{}, &[", const_name, image_path).unwrap();
            for rect in rects {
                let (x, y, w, h): (i64, i64, i64, i64) = rect.clone().try_into().unwrap();
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
