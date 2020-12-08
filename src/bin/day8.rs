use aoc2020::read_string_input;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct ProgramState {
    acc: i32,
    next_instruction: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum InstructionCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    code: InstructionCode,
    val: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ' ').collect();
        let val = parts[1].parse::<i32>().unwrap();
        let code = match parts[0] {
            "acc" => InstructionCode::Acc,
            "nop" => InstructionCode::Nop,
            "jmp" => InstructionCode::Jmp,
            _ => return Err("Invalid instruction".to_string()),
        };
        Ok(Instruction { code, val })
    }
}

impl Instruction {
    fn execute(&self, state: &ProgramState) -> ProgramState {
        match self.code {
            InstructionCode::Acc => ProgramState {
                acc: state.acc + self.val,
                next_instruction: state.next_instruction + 1,
            },
            InstructionCode::Nop => ProgramState {
                acc: state.acc,
                next_instruction: state.next_instruction + 1,
            },
            InstructionCode::Jmp => ProgramState {
                acc: state.acc,
                next_instruction: state.next_instruction + self.val,
            },
        }
    }
}

fn main() {
    let contents = read_string_input("in-data/day8.txt");
    let program = parse_input(&contents);

    println!("Part 1: {}", part_1(&program));
    println!("Part 2: {}", part_2(&program));
}

fn part_1(program: &Vec<Instruction>) -> i32 {
    let mut visited_addresses: HashSet<i32> = HashSet::new();
    let mut pointer = ProgramState {
        acc: 0,
        next_instruction: 0,
    };

    while !visited_addresses.contains(&pointer.next_instruction) {
        visited_addresses.insert(pointer.next_instruction);
        let instruction = &program[pointer.next_instruction as usize];
        pointer = instruction.execute(&pointer);
    }
    pointer.acc
}

fn part_2(program: &Vec<Instruction>) -> i32 {
    let mut program = program.clone();
    let mut candidate_instructions: Vec<usize> = Vec::new();
    for (index, instruction) in program.iter().enumerate() {
        match instruction.code {
            InstructionCode::Acc => {
                continue;
            }
            _ => {
                candidate_instructions.push(index);
            }
        }
    }

    for index in candidate_instructions {
        // modify instruction
        match program[index].code {
            InstructionCode::Nop => program[index].code = InstructionCode::Jmp,
            InstructionCode::Jmp => program[index].code = InstructionCode::Nop,
            _ => {}
        }

        // try to run program
        let mut visited_addresses: HashSet<i32> = HashSet::new();
        let mut pointer = ProgramState {
            acc: 0,
            next_instruction: 0,
        };
        while !visited_addresses.contains(&pointer.next_instruction) {
            visited_addresses.insert(pointer.next_instruction);
            let instruction = &program[pointer.next_instruction as usize];
            pointer = instruction.execute(&pointer);
            if pointer.next_instruction as usize == program.len() {
                // Found it!
                return pointer.acc;
            }
        }
        // reset instruction
        match program[index].code {
            InstructionCode::Nop => program[index].code = InstructionCode::Jmp,
            InstructionCode::Jmp => program[index].code = InstructionCode::Nop,
            _ => {}
        }
    }

    0
}

fn parse_input(contents: &str) -> Vec<Instruction> {
    contents
        .split('\n')
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_instruction() {
        assert_eq!(
            "acc +1".parse::<Instruction>().unwrap(),
            Instruction {
                code: InstructionCode::Acc,
                val: 1
            }
        );

        assert_eq!(
            "nop -1".parse::<Instruction>().unwrap(),
            Instruction {
                code: InstructionCode::Nop,
                val: -1
            }
        );

        assert_eq!(
            "jmp +4".parse::<Instruction>().unwrap(),
            Instruction {
                code: InstructionCode::Jmp,
                val: 4
            }
        );
    }

    #[test]
    fn test_sample_input_part_1() {
        let sample_input = parse_input(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );

        assert_eq!(part_1(&sample_input), 5);
    }

    #[test]
    fn test_sample_input_part_2() {
        let sample_input = parse_input(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );

        assert_eq!(part_2(&sample_input), 8);
    }
}
