use std::{
    fs::{self, File, ReadDir},
    io::Write,
    ffi::OsStr,
};

use toml::from_str;
use serde_xml_rs::deserialize;

use super::schema::*;

pub fn write_tile_grids<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            writeln!(file, "pub mod {} {{", path.file_name().unwrap().to_str().unwrap().to_owned().to_lowercase()).unwrap();
            writeln!(file, "use super::{{tile_set, TileGrid, Tile, Point, Dimen, Wall, SceneBuilder}};").unwrap();
            let sub_paths = fs::read_dir(path).unwrap();
            write_tile_grids(file, sub_paths);
            writeln!(file, "}}").unwrap();
        } else if path.extension() == Some(&OsStr::new("toml")) {
            let name = path.file_stem().unwrap();
            let const_name = name.to_str().unwrap().to_owned().to_uppercase();
            let toml_str = fs::read_to_string(&path).unwrap();
            let tile_grid: TileGridSpec = from_str(&toml_str).unwrap();
            writeln!(
                file, 
                "lazy_static! {{ pub static ref {}: TileGrid = TileGrid::new({}, {}, vec![{}]); }}", 
                const_name, 
                tile_grid.offset.unwrap_or(Point { x: 0, y: 0 }),
                tile_grid.size,
                tile_grid
                    .tiles
                    .iter()
                    .map(|tile| {
                        match tile {
                            Tile::Some(set, index) => format!("Some(Tile{{tile_set: &tile_set::{}, index: {}}})", tile_grid.tile_sets[*set], index),
                            Tile::None => "None".to_owned()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            ).unwrap();
        } else if path.extension() == Some(&OsStr::new("tmx")) {
            let name = path.file_stem().unwrap();
            let mod_name = name.to_str().unwrap().to_owned().to_lowercase();
            let xml_file = File::open(&path).unwrap();
            let tiled_grid_spec: TiledTMXSpec = deserialize(&xml_file).unwrap();
            let tile_grid = tiled_grid_spec.resolve();
            writeln!(file, "pub mod {} {{", mod_name).unwrap();
            writeln!(file, "use super::{{tile_set, TileGrid, Tile, Point, Dimen, Wall, SceneBuilder}};").unwrap();
            for layer in &tile_grid.layers {
                let const_name = layer.name.to_uppercase();
                if const_name == "COLLISIONS" {
                    writeln!(file, "pub fn collisions<'a, 'b, 'c>(builder: SceneBuilder<'a, 'b, 'c>) -> SceneBuilder<'a, 'b, 'c> {{").unwrap();
                    writeln!(file, "builder").unwrap();
                    for (index, tile) in layer.tiles.iter().enumerate() {
                        if *tile != 0 {
                            let x = index as u32 % tile_grid.width * tile_grid.tileheight;
                            let y = index as u32 / tile_grid.width * tile_grid.tilewidth;
                            let width = tile_grid.tilewidth;
                            let height = tile_grid.tileheight;
                            writeln!(file, ".add_entity(Wall({}, {}, {}, {}))", x, y, width, height).unwrap();
                        }
                    }
                    writeln!(file, "}}").unwrap();
                } else {
                    writeln!(
                        file, 
                        "lazy_static! {{ pub static ref {}: TileGrid = TileGrid::new({}, {}, vec![{}]); }}", 
                        const_name, 
                        Point { x: 0, y: 0 },
                        Dimen { width: tile_grid.width, height: tile_grid.height },
                        layer.tiles
                            .iter()
                            .map(|index| {
                                if *index == 0 { 
                                    "None".to_owned() 
                                } else { 
                                    let set = tile_grid
                                        .tilesets
                                        .iter()
                                        .find(|set| *index >= set.firstgid && *index < set.firstgid + set.tilecount)
                                        .unwrap();
                                    format!(
                                        "Some(Tile{{tile_set:&tile_set::{},index:{}}})",
                                        set.name,
                                        index - set.firstgid,
                                    )
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(",")
                        ).unwrap();
                }
            }
            writeln!(file, "}}").unwrap();
        }
    }
}
