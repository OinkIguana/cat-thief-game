use std::{
    fs::{self, ReadDir},
    io::Write,
    ffi::OsStr,
};

pub fn write_images<'a, W: Write>(file: &mut W, paths: ReadDir) {
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
