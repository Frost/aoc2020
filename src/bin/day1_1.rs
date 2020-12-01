use std::fs;
use std::vec;
use std::io;

fn main() -> std::io::Result<()> {
    let filename = "in-data/day1.1.txt";
    let mut data = read_input(filename)?;

    while let Some(i) = data.pop() {
        for j in &data {
            if i + j == 2020 {
                println!("{} + {}: product is {}", i, j, i * j);
                return Ok(())
            }
        }
    }
    Ok(())
}

fn read_input(filename: &str) -> io::Result<vec::Vec<i32>> {
    let contents = fs::read_to_string(filename)?;
    let str_array = contents.trim().split('\n').collect::<Vec<&str>>();
    Ok(str_array.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
}
