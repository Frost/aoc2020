use std::fs;

pub fn read_string_input(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => String::from(contents.trim()),
        _ => panic!("Error reading input"),
    }
}

// pub fn read_string_chunks<'a>(filename: &str, separator: Option<&str>) -> Vec<&'a str> {
//     let sep = match separator {
//         Some(s) => s,
//         None => "\n"
//     };

//     let contents = read_string_input(filename);
//     contents.split(sep).collect::<Vec<_>>()
// }

pub fn read_i32_tokens(filename: &str) -> Vec<i32> {
    read_string_input(filename)
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>()
}
