use aoc2020::read_string_input;
use regex::Regex;
use std::cmp::Ordering;

// --- Day 5: Binary Boarding ---
//
// You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.
//
// You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
//
// Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".
//
// The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.
//
// For example, consider just the first seven characters of FBFBBFFRLR:
//
//     Start by considering the whole range, rows 0 through 127.
//     F means to take the lower half, keeping rows 0 through 63.
//     B means to take the upper half, keeping rows 32 through 63.
//     F means to take the lower half, keeping rows 32 through 47.
//     B means to take the upper half, keeping rows 40 through 47.
//     B keeps rows 44 through 47.
//     F keeps rows 44 through 45.
//     The final F keeps the lower of the two, row 44.
//
// The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.
//
// For example, consider just the last 3 characters of FBFBBFFRLR:
//
//     Start by considering the whole range, columns 0 through 7.
//     R means to take the upper half, keeping columns 4 through 7.
//     L means to take the lower half, keeping columns 4 through 5.
//     The final R keeps the upper of the two, column 5.
//
// So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
//
// Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.
//
// Here are some other boarding passes:
//
//     BFFFBBFRRR: row 70, column 7, seat ID 567.
//     FFFBBBFRRR: row 14, column 7, seat ID 119.
//     BBFFBBFRLL: row 102, column 4, seat ID 820.

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

/// --- Part One ---
///
/// > As a sanity check, look through your list of boarding passes.
/// > What is the highest seat ID on a boarding pass?
///
/// Since I started by sorting all the boarding passes in `main()`, finding the
/// one with the highest id is simply a matter of unwrapping the first element
/// in the reversed vec.
fn part_1(boarding_passes: &[BoardingPass]) -> u32 {
    boarding_passes.iter().rev().next().unwrap().id
}

/// --- Part Two ---
///
/// > Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
///
/// > It's a completely full flight, so your seat should be the only missing
/// > boarding pass in your list. However, there's a catch: some of the seats at
/// > the very front and back of the plane don't exist on this aircraft, so
/// > they'll be missing from your list as well.
///
/// > Your seat wasn't at the very front or back, though; the seats with IDs +1
/// > and -1 from yours will be in your list.
///
/// > What is the ID of your seat?
///
/// The solution to this is once again dependent on the fact that I sorted the
/// list of boarding passes in `main()`. It uses a memo of the last visited
/// BoardingPass, and checks if the id on the current one is exactly the last
/// one plus 1. If it is not, that means we found the gap, which also means that
/// we found our seat.
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
