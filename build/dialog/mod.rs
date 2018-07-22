use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    io::Write,
    ffi::OsStr,
};

use toml::from_str;

use super::schema::*;
mod pretty_string;
use self::pretty_string::*;

pub fn format_attrs(attrs: &Vec<Vec<Attribute>>) -> String {
    let mut mapped: HashMap<&String, &String> = HashMap::default();
    for set in attrs {
        for attr in set {
            mapped.insert(&attr.name, &attr.value);
        }
    }
    mapped
        .into_iter()
        .map(|(key, value)| format!("Attribute::{}({})", key, value))
        .collect::<Vec<String>>()
        .join(",")
}

pub fn write_dialogs<'a, W: Write>(file: &mut W, paths: ReadDir) {
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            writeln!(file, "pub mod {} {{", path.file_name().unwrap().to_str().unwrap().to_owned().to_lowercase()).unwrap();
            writeln!(file, "use super::{{Color, font, Message, PrettyString, Attribute, DialogMessages}};").unwrap();
            let sub_paths = fs::read_dir(path).unwrap();
            write_dialogs(file, sub_paths);
            writeln!(file, "}}").unwrap();
        } else if path.extension() == Some(&OsStr::new("toml")) {
            let name = path.file_stem().unwrap();
            let const_name = name.to_str().unwrap().to_owned();
            let toml_str = fs::read_to_string(&path).unwrap();
            let dialog: DialogSpec = from_str(&toml_str).unwrap();
            writeln!(file, "pub fn {}(dialog: &mut DialogMessages) {{", const_name).unwrap();
            for message in dialog.messages {
                writeln!(file, "dialog.add(").unwrap();
                match message.speaker {
                    Some(speaker) => writeln!(file, "Message::new(String::from({:?}),", speaker).unwrap(),
                    None => writeln!(file, "Message::anon(").unwrap(),
                }
                writeln!(file, "PrettyString::new()").unwrap();
                let mut segment = String::new();
                let mut attributes = vec![];
                for token in tokenize(message.message) {
                    match token {
                        Token::StartSegment(name) => {
                            if !segment.is_empty() {
                                writeln!(file, ".add(({:?}, vec![{}]))", segment, format_attrs(&attributes)).unwrap();
                                segment = String::new();
                            }
                            if let Some(rule) = dialog.rules.iter().find(|rule| rule.name == name) {
                                attributes.push(rule.attributes.clone());
                            } else {
                                panic!("Undefined rule {} in dialog {}", name, const_name);
                            }
                        }
                        Token::EndSegment => {
                            if !segment.is_empty() {
                                writeln!(file, ".add(({:?}, vec![{}]))", segment, format_attrs(&attributes)).unwrap();
                                segment = String::new();
                            }
                            attributes.pop().expect("Ran out of attributes to pop");
                        }
                        Token::Text(text) => segment = segment + &text,
                    }
                }
                if !segment.is_empty() {
                    writeln!(file, ".add(({:?}, vec![{}]))", segment, format_attrs(&attributes)).unwrap();
                }
                writeln!(file, "));").unwrap();
            }
            writeln!(file, "}}").unwrap();
        }
    }
}
