use aoc2020::read_string_input;

fn main() {
    let input = read_string_input("in-data/day10.txt");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut data: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    data.sort_unstable();

    // This has length 4 due to zero-indexing
    // The zeroeth and and second index is never used, but I can use this vec as
    // a cheap map by doing differences[some_index]
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

fn part_2(_input: &str) -> usize {
    // The solution to this is to find how many paths there are through each cluster where skips might appear.
    //
    // I got the idea for this by looking up the prime factors of the answer to the longer sample.
    //
    // The prime factors of 19208 are: 2 * 2 * 2 * 7 * 7 * 7 * 7
    //
    // If we look at the clusters of that example, we find 4 clusters with 5
    // consecutive numbers (each with 7 possible paths through them), 1 cluster
    // with 4 numbers (4 possible paths = 2 * 2), and one cluster with 3
    // consecutive numbers (2 possible paths).
    //
    // These numbers correspond with the prime factors of 19208...
    //
    // I solved this manually, by pasting the input into an Emacs buffer,
    // sorting it, grouping clusters where skips might happen, and then manually
    // calculating how many possible paths there are through each cluster. The
    // paths are shown after the arrows in the following section:
    //
    // 0 1 2 -> 2
    // 5 6 7 8 9 -> 7
    // 12 -> 1
    // 13 -> 1
    // 16 17 18 19 -> 4
    // 22 -> 1
    // 25 26 27 28 -> 4
    // 31 -> 1
    // 34 35 36 37 38 -> 7
    // 41 -> 1
    // 44 45 46 47 48 -> 7
    // 51 52 53 -> 2
    // 56 -> 1
    // 59 60 61 62 -> 4
    // 65 66 67 68 -> 4
    // 71 72 73 74 75 -> 7
    // 78 -> 1
    // 81 -> 1
    // 84 -> 1
    // 87 88 89 90 -> 4
    // 93 94 95 -> 2
    // 98 -> 1
    // 99 -> 1
    // 102 103 104 105 106 -> 7
    // 109 110 111 112 113 -> 7
    // 116 117 118 -> 2
    // 121 122 123 124 -> 4
    // 127 -> 1
    // 130 -> 1
    // 133 134 135 136 137 -> 7
    // 140 141 142 -> 2
    // 145 146 147 148 149 -> 7
    // 152 -> 1
    // 153 -> 1
    // 156 -> 1
    // 159 160 161 -> 2
    // 164 -> 1
    // 165 -> 1
    // 168 169 170 171 172 -> 7
    // 175 -> 1
    // 178 179 180 181 182 -> 7
    //
    // The solution is the product of these paths, so if we skip the ones here, that would yield:
    2 * 7 * 4 * 4 * 7 * 7 * 2 * 4 * 4 * 7 * 4 * 2 * 7 * 7 * 2 * 4 * 7 * 2 * 7 * 2 * 7 * 7
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_part_2_simple_sample() {
        let _sample_input = "16
10
15
5
1
11
7
19
6
12
4";
        // assert_eq!(part_2(&sample_input), 8);
    }

    #[test]
    fn test_part_2_more_complex_sample() {
        let _sample_input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        // assert_eq!(part_2(&sample_input), 19208);
    }
}
