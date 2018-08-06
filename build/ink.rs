use inkgen::{parse, pretty_print};
use std::io::Write;
use std::ffi::OsStr;
use std::fs::{read_to_string, read_dir, ReadDir};

pub fn write_inks<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            writeln!(file, "pub mod {} {{", path.file_name().unwrap().to_str().unwrap().to_owned().to_lowercase()).unwrap();
            let sub_paths = read_dir(path).unwrap();
            write_inks(file, sub_paths);
            writeln!(file, "}}").unwrap();
        } else if path.extension() == Some(&OsStr::new("ink")) {
            let name = path.file_stem().unwrap().to_str().unwrap();

            let string = read_to_string(&path).expect("The input did not contain valid UTF-8");
            let ink = parse(string).unwrap();
            let generated = pretty_print(
                name,
                ink,
            );
            writeln!(file, "{}", generated);
        }
    }
}
