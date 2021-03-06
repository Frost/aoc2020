use aoc2020::read_string_input;

fn main() {
    let filename = "in-data/day3.txt";
    let data = read_string_input(filename);

    let trees_1 = traverse_data(&data, 1, 1);
    let trees_2 = traverse_data(&data, 1, 3);
    let trees_3 = traverse_data(&data, 1, 5);
    let trees_4 = traverse_data(&data, 1, 7);
    let trees_5 = traverse_data(&data, 2, 1);

    println!("Part 1: {}", trees_2);
    println!(
        "Part 2: {}",
        trees_1 * trees_2 * trees_3 * trees_4 * trees_5
    );
}

fn traverse_data(data: &str, row_diff: usize, col_diff: usize) -> u32 {
    let mut trees = 0;
    let mut index = 0;
    let mut row = 0;
    for line in data.lines() {
        if row % row_diff > 0 {
            row = (row + 1) % row_diff;
            continue;
        }
        row = (row + 1) % row_diff;
        let width = line.len();
        let maybe_tree = line.chars().nth(index).unwrap();
        if maybe_tree == '#' {
            trees += 1;
        }

        index = (index + col_diff) % width;
    }

    trees
}
