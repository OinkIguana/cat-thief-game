use std::{
    fs::{self, ReadDir},
    io::Write,
    ffi::OsStr,
};

use toml::from_str;

use super::schema::*;

pub fn write_tile_sets<'a, W: Write>(file: &mut W, paths: ReadDir) {
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

