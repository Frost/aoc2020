use aoc2020::read_string_input;

fn main() {
    let input = read_string_input("in-data/day10.txt");

    println!("Part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    let mut data: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    data.sort_unstable();

    // this has length 4 due to zero-indexing
    let mut differences: Vec<usize> = vec![0, 0, 0, 0];

    let mut last_jolts_spec = 0;

    for number in data {
        let diff = number - last_jolts_spec;
        differences[diff] += 1;
        last_jolts_spec = number;
    }

    differences[3] += 1;

    differences[1] * differences[3]
}
