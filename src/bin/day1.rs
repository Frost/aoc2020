use aoc2020::read_i32_tokens;

fn main() {
    let filename = "in-data/day1.txt";
    let data = read_i32_tokens(filename);

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
