use aoc2020::read_string_input;
use std::collections::VecDeque;
use std::str::FromStr;

//
// The idea here is to build up a vec with structs for the <preamble size> last numbers
//
// Each of those structs should contain all numbers valid in the current data chunk because of this number.
//
// Given sample input data like "1 2 3 4 5" and a preamble size of 5, that means:
//
// [
//   Number {
//     number: 1,
//     valid_numbers: [(1 + 2), (1 + 3), (1 + 4), (1 + 5)]
//   },
//   Number {
//     number: 2,
//     valid_numbers: [(2 + 3), (2 + 4), (2 + 5)]
//   },
//   Number {
//     number: 3,
//     valid_numbers: [ (3 + 4), (3 + 5)]
//   },
//   Number {
//     number: 4,
//     valid_numbers: [ (4 + 5) ]
//   },
//   Number {
//     number: 5,
//     valid_numbers: []
//   }
// ]
//
// This means it will generate a triangle of valid numbers.
// The reason for doing it this way, is that when we process a new number, the process becomes:
//
//  * Discard the first struct (since anything in that is now invalid anyway)
//  * Iterate through the rest of the list, pushing (current_number +
//    number_to_be_added) to each structs `valid_numbers` list
//  * Add Number { number: number_to_be_added, valid_numbers: [] } to the end
//
//  Checking validity for a number can be done by iterating through the list and
//  seeing if the new number is present in any of the `valid_numbers` lists.

#[derive(Debug)]
struct XMAS {
    preamble: usize,
    data: VecDeque<Entry>,
}

#[derive(Debug)]
struct Entry {
    number: i64,
    valid_numbers: Vec<i64>,
}

impl From<i64> for Entry {
    fn from(number: i64) -> Self {
        Entry {
            number,
            valid_numbers: Vec::new(),
        }
    }
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse::<i64>().unwrap();
        Ok(Entry {
            number,
            valid_numbers: Vec::new(),
        })
    }
}

impl XMAS {
    fn new(preamble: usize) -> XMAS {
        XMAS {
            preamble,
            data: VecDeque::new(),
        }
    }
    /// Parse a &str token and add it to &self.data
    fn add_entry(&mut self, new_entry: Entry) {
        if self.data.len() >= self.preamble {
            self.data.pop_front();
        }
        for entry in self.data.iter_mut() {
            entry.valid_numbers.push(entry.number + new_entry.number);
        }
        self.data.push_back(new_entry);
    }

    fn valid_number(&self, entry: &Entry) -> bool {
        for e in &self.data {
            if e.valid_numbers.contains(&entry.number) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let input = read_string_input("in-data/day9.txt");

    println!("Part 1: {}", part_1(&input, 25));
    println!("Part 2: {}", part_2(&input, 25));
}

fn part_1(input: &str, preamble_size: usize) -> i64 {
    let mut xmas = XMAS::new(preamble_size);
    let data = input.split_ascii_whitespace().collect::<Vec<&str>>();

    let preamble = data.iter().take(preamble_size);
    for token in preamble {
        if let Ok(entry) = token.parse::<Entry>() {
            xmas.add_entry(entry);
        }
    }

    for token in data.iter().skip(preamble_size) {
        if let Ok(number) = token.parse::<i64>() {
            let entry = Entry::from(number);
            if xmas.valid_number(&entry) {
                xmas.add_entry(entry);
            } else {
                return number;
            }
        }
    }
    0
}

// --- Part Two ---

// The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.

// Again consider the above example:

// 35
// 20
// 15
// 25
// 47
// 40
// 62
// 55
// 65
// 95
// 102
// 117
// 150
// 182
// 127
// 219
// 299
// 277
// 309
// 576

// In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)

// To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.

// What is the encryption weakness in your XMAS-encrypted list of numbers?
fn part_2(input: &str, preamble_size: usize) -> i64 {
    let invalid_number = part_1(input, preamble_size);

    let mut data: Vec<i64> = vec![];
    for token in input.split_ascii_whitespace() {
        data.push(token.parse::<i64>().unwrap());
    }
    // let data = input.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i64>>();

    let mut range: Vec<&i64> = vec![];
    for (index, item) in data.iter().enumerate() {
        range = vec![item];
        let mut sum = *item;
        for other in data.iter().skip(index + 1) {
            if sum + other > invalid_number {
                break;
            }
            range.push(other);
            sum += other;
        }
        if sum == invalid_number {
            break;
        }
    }

    range.sort();

    let smallest = range.iter().min().unwrap();
    let largest = range.iter().max().unwrap();

    *smallest + *largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_1_25() {
        //     For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:

        // 26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
        // 49 would be a valid next number, as it is the sum of 24 and 25.
        // 100 would not be valid; no two of the previous 25 numbers sum to 100.
        // 50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.
        let sample_input = "1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25";
        let mut xmas = XMAS::new(25);
        for token in sample_input.split_ascii_whitespace() {
            let entry = token.parse::<Entry>().unwrap();
            xmas.add_entry(entry);
        }

        assert!(xmas.valid_number(&Entry::from(26)));
        assert!(xmas.valid_number(&Entry::from(49)));
        assert!(!xmas.valid_number(&Entry::from(100)));
        assert!(!xmas.valid_number(&Entry::from(50)));
    }

    #[test]
    fn test_1_to_25_with_20_first() {
        // Suppose the 26th number is 45, and the first number (no longer an
        // option, as it is more than 25 numbers ago) was 20. Now, for the next
        // number to be valid, there needs to be some pair of numbers among
        // 1-19, 21-25, or 45 that add up to it:

        // 26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
        // 65 would not be valid, as no two of the available numbers sum to it.
        // 64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
        let sample_input = "20
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
21
22
23
24
25";
        let mut xmas = XMAS::new(25);
        for token in sample_input.split_ascii_whitespace() {
            let entry = token.parse::<Entry>().unwrap();
            xmas.add_entry(entry);
        }

        xmas.add_entry(Entry::from(45));

        dbg!(&xmas);
        assert!(xmas.valid_number(&Entry::from(26)));
        assert!(!xmas.valid_number(&Entry::from(65)));
        assert!(xmas.valid_number(&Entry::from(64)));
        assert!(xmas.valid_number(&Entry::from(66)));
    }

    #[test]
    fn test_preamble_size_5() {
        let sample_data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        assert_eq!(part_1(&sample_data, 5), 127);
        // let sample_data = sample_data.split_ascii_whitespace();
        // let mut data = read_preamble(&sample_data, 5);

        // Do something here...
        // How to continue processing input?
    }
}
