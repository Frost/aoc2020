use aoc2020::read_string_input;
use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct BoardingPass {
    row: u32,
    col: u32,
    id: u32,
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.id == other.id
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let data = read_string_input("in-data/day5.txt");
    let mut boarding_passes = data
        .lines()
        .map(parse_boarding_pass)
        .collect::<Vec<BoardingPass>>();
    boarding_passes.sort();

    println!("Part 1: {}", part_1(&boarding_passes));
    println!("Part 2: {}", part_2(&boarding_passes).unwrap());
}

fn part_1(boarding_passes: &[BoardingPass]) -> u32 {
    boarding_passes.iter().rev().next().unwrap().id
}

fn part_2(boarding_passes: &[BoardingPass]) -> Option<u32> {
    let mut last_visited: Option<&BoardingPass> = None;

    for bp in boarding_passes.iter() {
        match last_visited {
            None => {}
            Some(last_visited) => {
                let maybe_current_id = last_visited.id + 1;
                if maybe_current_id < bp.id {
                    // We found the place where there's a gap, that's our seat
                    return Some(maybe_current_id);
                }
            }
        }
        last_visited = Some(bp);
    }

    None
}

fn parse_boarding_pass(pass: &str) -> BoardingPass {
    let re = Regex::new(r"^(?P<row>[FB]{7})(?P<col>[RL]{3})").unwrap();
    let captures = re.captures(pass).unwrap();
    let row = translate_to_binary(&captures.name("row").unwrap().as_str());
    let col = translate_to_binary(&captures.name("col").unwrap().as_str());

    BoardingPass {
        row,
        col,
        id: row * 8 + col,
    }
}

fn translate_to_binary(part: &str) -> u32 {
    let binary_string = part
        .replace("B", "1")
        .replace("F", "0")
        .replace("R", "1")
        .replace("L", "0");

    u32::from_str_radix(&binary_string, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_row_col() {
        assert!(
            parse_boarding_pass(&"BFFFBBFRRR")
                == BoardingPass {
                    row: 70,
                    col: 7,
                    id: 567,
                }
        );

        assert!(
            parse_boarding_pass(&"FFFBBBFRRR")
                == BoardingPass {
                    row: 14,
                    col: 7,
                    id: 119,
                }
        );

        assert!(
            parse_boarding_pass(&"BBFFBBFRLL")
                == BoardingPass {
                    row: 102,
                    col: 4,
                    id: 820,
                }
        );
    }

    #[test]
    fn test_boarding_pass_ordering() {
        let mut boarding_passes = vec![
            parse_boarding_pass(&"BFFFBBFRRR"), // id: 567
            parse_boarding_pass(&"FFFBBBFRRR"), // id: 119
            parse_boarding_pass(&"BBFFBBFRLL"), // id: 820
        ];

        boarding_passes.sort();

        assert_eq!(
            boarding_passes,
            vec![
                parse_boarding_pass(&"FFFBBBFRRR"), // id: 119
                parse_boarding_pass(&"BFFFBBFRRR"), // id: 567
                parse_boarding_pass(&"BBFFBBFRLL"), // id: 820
            ]
        );
    }
}
