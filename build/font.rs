use std::{
    path::PathBuf,
    fs::{self, ReadDir},
    io::Write,
    ffi::OsStr,
};

use toml::from_str;

use super::schema::*;

pub fn write_fonts<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if !path.is_dir() && path.extension() == Some(&OsStr::new("toml")) {
            let name = path.file_stem().unwrap();
            writeln!(file, "pub mod {} {{", name.to_str().unwrap().to_owned()).unwrap();
            writeln!(file, "use super::Font;").unwrap();
            let toml_str = fs::read_to_string(&path).unwrap();
            let fonts: FontSpec = from_str(&toml_str).unwrap();
            for font in fonts.styles {
                let mut file_path = PathBuf::from("src/font");
                file_path.push(font.file);
                for size in font.sizes {
                    let const_name = format!("{}_{}", font.name.to_uppercase(), size);
                    writeln!(file, "pub const {}: Font = Font::new({:?}, {});", const_name, file_path, size).unwrap();
                }
            }
            writeln!(file, "}}").unwrap();
        }
    }
}
