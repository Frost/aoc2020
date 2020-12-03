use std::fs;
use std::vec;
use std::io;

struct Policy {
    pub min: usize,
    pub max: usize,
    pub letter: char,
}

struct Password {
    pub policy: Policy,
    pub password: String,
}

fn main() -> io::Result<()> {
    let data = read_input("in-data/day2.txt")?;
    let valid_passwords = data.iter().filter(|p| check_password_validity(&p));
    println!("{:?}", valid_passwords.count());
    let valid_passwords_2 = data.iter().filter(|p| check_password_validity_2(&p));
    println!("{:?}", valid_passwords_2.count());
    Ok(())
}

fn read_input(filename: &str) -> io::Result<vec::Vec<Password>> {
    let contents = fs::read_to_string(filename)?;
    let str_array = contents.trim().split('\n').collect::<vec::Vec<&str>>();
    Ok(str_array.iter().map(parse_password).collect::<vec::Vec<Password>>())
}

fn parse_password(line: &&str) -> Password {
    let parts = line.trim().split(' ').collect::<vec::Vec<&str>>();
    let minmax = parts[0].split('-').collect::<vec::Vec<&str>>();
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
