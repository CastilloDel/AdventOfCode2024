use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{satisfy, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    AsChar, IResult,
};

fn main() {
    let contents = fs::read_to_string("test_input_part2").unwrap();
    let result = day17_part1(&contents);
    println!("Day17 part 1 result: {result}");
    let result = day17_part2(&contents);
    println!("Day17 part 2 result: {result}");
}

fn day17_part1(input: &str) -> String {
    let (_, computer) = read_input(input).unwrap();
    execute_instructions(computer)
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn day17_part2(input: &str) -> usize {
    let (_, computer) = read_input(input).unwrap();
    find_replicating_program(computer)
}

fn execute_instructions(mut computer: Computer) -> Vec<u64> {
    loop {
        if !computer.execute_instruction() {
            break;
        }
    }
    computer.output
}

fn find_replicating_program(computer: Computer) -> usize {
    // 0 (2,4) => b = a % 8
    // 2 (1,2) => b = b ^ 2     b [0,1,4,5]
    // 4 (7,5) => c = a >> b   c 0..inf
    // 6 (4,7) => b = b ^ c     b
    // 8 (1,3) => b = b ^ 3
    // 8 (5,5) => output b % 8
    // 10 (0,3) => a = a / 8
    // 12 (3,0) => start again

    let mut starters = (0..2_usize.pow(10)).collect::<Vec<_>>();

    for i in 0..computer.instructions.len() {
        starters = starters
            .iter()
            .flat_map(|starter| (0..8).map(move |v| (starter + (v << (i * 3 + 10)))))
            .filter(|&starter| {
                let mut computer_test = computer.clone();
                computer_test.a = starter as u64;
                let received = execute_instructions(computer_test);
                received.len() > i && received[0..=i] == computer.instructions[0..=i]
            })
            .collect();
    }
    starters.into_iter().min().unwrap()
}

#[derive(Debug, Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    instruction_pointer: usize,
    instructions: Vec<u64>,
    output: Vec<u64>,
}

impl Computer {
    fn read_combo_operand(&self, operand: u64) -> u64 {
        match operand {
            i if i < 4 => i,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    fn execute_instruction(&mut self) -> bool {
        if self.instruction_pointer >= self.instructions.len() - 1 {
            return false;
        }
        let instruction = self.instructions[self.instruction_pointer];
        let operand = self.instructions[self.instruction_pointer + 1];
        match instruction {
            0 => self.a /= 2_u64.pow(self.read_combo_operand(operand) as u32),
            1 => self.b ^= operand,
            2 => self.b = self.read_combo_operand(operand) % 8,
            3 => {
                if self.a != 0 {
                    self.instruction_pointer = operand as usize;
                    return true;
                }
            }
            4 => self.b ^= self.c,
            5 => self.output.push(self.read_combo_operand(operand) % 8),
            6 => self.b = self.a / 2_u64.pow(self.read_combo_operand(operand) as u32),
            7 => self.c = self.a / 2_u64.pow(self.read_combo_operand(operand) as u32),
            _ => unreachable!(),
        }
        self.instruction_pointer += 2;
        true
    }
}

fn read_input(input: &str) -> IResult<&str, Computer> {
    map(
        tuple((
            read_register,
            read_register,
            read_register,
            read_instructions,
        )),
        |(a, b, c, instructions)| Computer {
            a,
            b,
            c,
            instructions,
            instruction_pointer: 0,
            output: Vec::new(),
        },
    )(input)
}

fn read_register(input: &str) -> IResult<&str, u64> {
    preceded(many1(satisfy(|c| !c.is_dec_digit())), u64)(input)
}

fn read_instructions(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        many1(satisfy(|c| !c.is_dec_digit())),
        separated_list1(tag(","), u64),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input_part1() {
        let contents = fs::read_to_string("test_input_part1").unwrap();
        let result = day17_part1(&contents);
        assert_eq!(result, String::from("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day17_part1(&contents);
        assert_eq!(result, String::from("2,7,4,7,2,1,7,5,1"));
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day17_part2(&contents);
        assert_eq!(result, 37221274271220);
    }
}
