use aoc2020::read_string_input;
use std::collections::HashSet;

fn main() {
    let data = read_input("in-data/day6.txt");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_input(filename: &str) -> Vec<Vec<HashSet<String>>> {
    read_string_input(filename)
        .split("\n\n")
        .map(parse_group)
        .collect()
}

fn part_1(data: &Vec<Vec<HashSet<String>>>) -> usize {
    data.iter().map(union_group).map(|s| s.len()).sum()
}

fn part_2(data: &Vec<Vec<HashSet<String>>>) -> usize {
    data.iter().map(intersect_group).map(|s| s.len()).sum()
}

fn parse_group(group: &str) -> Vec<HashSet<String>> {
    group.trim().split('\n').map(parse_person).collect()
}

fn parse_person(data: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    for c in data.chars() {
        set.insert(c.to_string());
    }
    set
}

fn union_group(group: &Vec<HashSet<String>>) -> HashSet<String> {
    let mut set = HashSet::new();
    for person in group {
        set = set.union(person).cloned().collect();
    }
    set
}

fn intersect_group(group: &Vec<HashSet<String>>) -> HashSet<String> {
    let mut set: HashSet<String> = "abcdefghijklmnopqrstuvwxyz"
        .split("")
        .map(|c| String::from(c))
        .collect();
    for person in group {
        set = set.intersection(person).cloned().collect();
    }
    set
}
