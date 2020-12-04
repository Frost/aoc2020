use std::fs;

pub fn read_string_input(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => String::from(contents.trim()),
        _ => panic!("Error reading input")
    }
}
