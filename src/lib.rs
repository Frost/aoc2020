use std::fs;

pub fn read_string_input(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => String::from(contents.trim()),
        _ => panic!("Error reading input")
    }
}

pub fn read_i32_tokens(filename: &str) -> Vec<i32> {
    read_string_input(filename).lines().map(|s| s.parse().unwrap()).collect::<Vec<i32>>()
}
