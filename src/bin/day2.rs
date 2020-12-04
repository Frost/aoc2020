use aoc2020::read_string_input;

struct Policy {
    pub min: usize,
    pub max: usize,
    pub letter: char,
}

struct Password {
    pub policy: Policy,
    pub password: String,
}

fn main() {
    let data = read_input("in-data/day2.txt");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_input(filename: &str) -> Vec<Password> {
    read_string_input(filename).lines().map(parse_password).collect::<Vec<Password>>()
}

fn part_1(data: &[Password]) -> usize {
    data.iter().filter(|p| check_password_validity(&p)).count()
}

fn part_2(data: &[Password]) -> usize {
    data.iter().filter(|p| check_password_validity_2(&p)).count()
}

fn parse_password(line: &str) -> Password {
    let parts = line.trim().split(' ').collect::<Vec<&str>>();
    let minmax = parts[0].split('-').collect::<Vec<&str>>();
    let policy = Policy {
        min: minmax[0].parse::<usize>().unwrap(),
        max: minmax[1].parse::<usize>().unwrap(),
        letter: parts[1].chars().next().unwrap(),
    };
    Password {
        policy,
        password: parts[2].to_string(),
    }
}

fn check_password_validity(password: &Password) -> bool {
    let policy = &password.policy;
    let valid_chars = password.password.chars().filter(|c| c == &policy.letter);
    let num_valid_chars = valid_chars.count();
    policy.min <= num_valid_chars && num_valid_chars <= policy.max
}

fn check_password_validity_2(password: &Password) -> bool {
    let policy = &password.policy;
    let first_char = password.password.chars().nth(policy.min - 1).unwrap();
    let second_char = password.password.chars().nth(policy.max - 1).unwrap();
    let valid_first_char = first_char == policy.letter;
    let valid_second_char = second_char == policy.letter;

    valid_first_char ^ valid_second_char
}
