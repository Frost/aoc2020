use aoc2020::read_string_input;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
    Invalid,
}

impl From<&str> for Position {
    fn from(c: &str) -> Position {
        match c {
            "." => Position::Floor,
            "L" => Position::Empty,
            "#" => Position::Occupied,
            _ => Position::Invalid,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Position::Floor => '.',
            Position::Empty => 'L',
            Position::Occupied => '#',
            Position::Invalid => '?',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq)]
struct SeatMap {
    positions: Vec<Vec<Position>>,
    rows: usize,
    cols: usize,
}

impl SeatMap {

    fn step(&mut self) {
        let mut new_positions: Vec<Vec<Position>> = Vec::new();

        for (y, row) in self.positions.iter().enumerate() {
            let mut new_row: Vec<Position> = Vec::new();
            for (x, position) in row.iter().enumerate() {
                let neighbors = self.occupied_neighbors(x, y);

                // If a seat is empty (L) and there are no occupied seats
                // adjacent to it, the seat becomes occupied.
                // If a seat is occupied (#) and four or more seats adjacent to
                // it are also occupied, the seat becomes empty.
                // Otherwise, the seat's state does not change.
                match position {
                    Position::Empty => {
                        if neighbors == 0 {
                            new_row.push(Position::Occupied);
                        } else {
                            new_row.push(Position::Empty);
                        }
                    },
                    Position::Occupied => {
                        if neighbors >= 4 {
                            new_row.push(Position::Empty);
                        } else {
                            new_row.push(Position::Occupied);
                        }
                    },
                    Position::Floor => {
                        new_row.push(Position::Floor);
                    },
                    Position::Invalid => {},
                }
            }
            new_positions.push(new_row);
        }

        self.positions = new_positions;
        self.rows = self.positions.len();
        self.cols = self.positions[0].len();
    }

    fn step_2(&mut self) {
        let mut new_positions: Vec<Vec<Position>> = Vec::new();

        for (y, row) in self.positions.iter().enumerate() {
            let mut new_row: Vec<Position> = Vec::new();
            for (x, position) in row.iter().enumerate() {
                let neighbors = self.occupied_visible_seats(x, y);

                // If a seat is empty (L) and there are no occupied seats
                // adjacent to it, the seat becomes occupied.
                // If a seat is occupied (#) and four or more seats adjacent to
                // it are also occupied, the seat becomes empty.
                // Otherwise, the seat's state does not change.
                match position {
                    Position::Empty => {
                        if neighbors == 0 {
                            new_row.push(Position::Occupied);
                        } else {
                            new_row.push(Position::Empty);
                        }
                    },
                    Position::Occupied => {
                        if neighbors >= 5 {
                            new_row.push(Position::Empty);
                        } else {
                            new_row.push(Position::Occupied);
                        }
                    },
                    Position::Floor => {
                        new_row.push(Position::Floor);
                    },
                    Position::Invalid => {},
                }
            }
            new_positions.push(new_row);
        }

        self.positions = new_positions;
        self.rows = self.positions.len();
        self.cols = self.positions[0].len();
    }

    fn occupied_neighbors(&self, x: usize, y: usize) -> usize {
        let mut ydiff: &[i32] = &[-1, 0, 1];
        let mut xdiff: &[i32] = &[-1, 0, 1];
        if y == 0 {
            ydiff = &[0, 1];
        } else if y == self.rows - 1 {
            ydiff = &[-1, 0];
        }
        if x == 0 {
            xdiff = &[0, 1];
        } else if x == self.cols - 1 {
            xdiff = &[-1, 0];
        }

        let mut neighbors = 0;
        for dy in ydiff {
            for dx in xdiff {
                let xi = (x as i32 + dx) as usize;
                let yi = (y as i32 + dy) as usize;

                if dx == &0 && dy == &0 {
                    continue;
                }

                if self.positions[yi][xi] == Position::Occupied {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn occupied_visible_seats(&self, x: usize, y: usize) -> usize {
        let mut ydiff: &[i32] = &[-1, 0, 1];
        let mut xdiff: &[i32] = &[-1, 0, 1];

        if y == 0 {
            ydiff = &[0, 1];
        } else if y == self.rows - 1 {
            ydiff = &[-1, 0];
        }
        if x == 0 {
            xdiff = &[0, 1];
        } else if x == self.cols - 1 {
            xdiff = &[-1, 0];
        }

        let mut neighbors = 0;

        for dy in ydiff {
            for dx in xdiff {
                if dx == &0 && dy == &0 {
                    continue;
                }
                // find closest neighbor in direction (dx, dy) and check it
                let mut xcheck = x as i32 + dx;
                let mut ycheck = y as i32 + dy;
                let mut p = &self.positions[y][x];
                while xcheck >= 0 && ycheck >= 0 && xcheck <= self.cols as i32 - 1 && ycheck <= self.rows as i32 - 1 {
                    p = &self.positions[ycheck as usize][xcheck as usize];
                    if p != &Position::Floor {
                        break;
                    }
                    xcheck += dx;
                    ycheck += dy;
                }
                if p == &Position::Occupied {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn occupied_seats(&self) -> usize {
        let mut occupied = 0;

        for row in &self.positions {
            for col in row {
                if col == &Position::Occupied {
                    occupied += 1;
                }
            }
        }
        occupied
    }
}

impl FromStr for SeatMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions: Vec<Vec<Position>> = Vec::new();

        for line in s.lines() {
            let mut row: Vec<Position> = line.split("").map(Position::from).collect();
            row.remove(0);
            row.pop();
            positions.push(row);
        }

        let rows = &positions.len();
        let cols = &positions[0].len();

        Ok(SeatMap {
            positions,
            rows: *rows,
            cols: *cols,
        })
    }
}

impl fmt::Display for SeatMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .positions
            .iter()
            .map(|l| {
                l.iter()
                    .map(Position::to_string)
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

fn main() {
    let input = read_string_input("in-data/day11.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut seat_map = SeatMap::from_str(input).unwrap();
    let mut seat_map_string = seat_map.to_string();

    loop {
        seat_map.step();
        if seat_map.to_string() == seat_map_string {
            break;
        }
        seat_map_string = seat_map.to_string();
    }
    seat_map.occupied_seats()
}

fn part_2(input: &str) -> usize {
    let mut seat_map = SeatMap::from_str(input).unwrap();
    let mut seat_map_string = seat_map.to_string();

    loop {
        seat_map.step_2();
        if seat_map.to_string() == seat_map_string {
            break;
        }
        seat_map_string = seat_map.to_string();
    }
    seat_map.occupied_seats()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_from_string() {
        assert_eq!(Position::from("."), Position::Floor);
        assert_eq!(Position::from("L"), Position::Empty);
        assert_eq!(Position::from("#"), Position::Occupied);
    }

    #[test]
    fn test_position_to_string() {
        assert_eq!(Position::Floor.to_string(), ".");
        assert_eq!(Position::Empty.to_string(), "L");
        assert_eq!(Position::Occupied.to_string(), "#");
    }

    #[test]
    fn test_subscript() {
        let sample_input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let seat_map = SeatMap::from_str(sample_input).unwrap();

        assert_eq!(seat_map.positions[0][0], Position::Empty);
        assert_eq!(seat_map.positions[0][1], Position::Floor);
        assert_eq!(seat_map.positions[9][9], Position::Empty);
    }

    #[test]
    fn test_occupied_neighbors() {
        let sample_input = "...
.##
...";
        let seat_map = SeatMap::from_str(sample_input).unwrap();
        assert_eq!(seat_map.occupied_neighbors(1, 1), 1);

        let sample_input = "#..
.L#
...";
        let seat_map = SeatMap::from_str(sample_input).unwrap();
        assert_eq!(seat_map.occupied_neighbors(1, 1), 2);
    }

    #[test]
    fn test_step() {
        let sample_input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let sample_step_2 = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

        let sample_step_3 = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

        let sample_step_4 = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";

        let sample_step_5 = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";

        let sample_step_6 = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

        let mut seat_map = SeatMap::from_str(sample_input).unwrap();

        seat_map.step();
        assert_eq!(&seat_map, &SeatMap::from_str(sample_step_2).unwrap());

        seat_map.step();
        assert_eq!(&seat_map, &SeatMap::from_str(sample_step_3).unwrap());

        seat_map.step();
        assert_eq!(&seat_map, &SeatMap::from_str(sample_step_4).unwrap());

        seat_map.step();
        assert_eq!(&seat_map, &SeatMap::from_str(sample_step_5).unwrap());

        seat_map.step();
        assert_eq!(&seat_map, &SeatMap::from_str(sample_step_6).unwrap());
        seat_map.step();
        assert_eq!(&seat_map, &SeatMap::from_str(sample_step_6).unwrap());
    }

    #[test]
    fn test_occupied_visible_seats() {
        let sample_input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

        let seat_map = SeatMap::from_str(sample_input).unwrap();

        assert_eq!(seat_map.occupied_visible_seats(3, 4), 8);
    }
}
