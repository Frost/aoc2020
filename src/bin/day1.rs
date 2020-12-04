use std::fs;
use std::vec;
use std::io;

fn main() {
    let filename = "in-data/day1.txt";
    let data = read_input(filename).expect("Error reading input");

    println!("Part 1: {}", part_1(&data).unwrap());
    println!("Part 2: {}", part_2(&data).unwrap());
}

fn part_1(data: &[i32]) -> Result<i32, &'static str> {
    for i in data {
        for j in data {
            if i + j == 2020 {
                return Ok(i * j)
            }
        }
    }
    Err("No result found")
}

fn part_2(data: &[i32]) -> Result<i32, &'static str> {
    for i in data {
        for j in data {
            for k in data {
                if i + j + k == 2020 {
                    return Ok(i * j * k)
                }
            }
        }
    }
    Err("No result found")
}

fn read_input(filename: &str) -> io::Result<vec::Vec<i32>> {
    let contents = fs::read_to_string(filename)?;
    let str_array = contents.trim().split('\n').collect::<Vec<&str>>();
    Ok(str_array.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
}
