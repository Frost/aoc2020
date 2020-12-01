use std::fs;
use std::vec;
use std::io;

fn main() -> std::io::Result<()> {
    let filename = "in-data/day1.1.txt";
    let data = read_input(filename)?;

    for i in &data {
        for j in &data {
            for k in &data {
                if i + j + k == 2020 {
                    println!("{} + {} + {}: product is {}", i, j, k, i * j * k);
                    return Ok(())
                }
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
