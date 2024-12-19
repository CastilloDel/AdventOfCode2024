use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, satisfy, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    AsChar, IResult,
};

fn main() {
    let contents = fs::read_to_string("message.txt").unwrap();
    let result = day17_part1(&contents);
    println!("Day17 part 1 result: {result}");
}

fn day17_part1(input: &str) -> String {
    let (_, computer) = read_input(input).unwrap();
    execute_instructions(computer)
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn execute_instructions(mut computer: Computer) -> Vec<u64> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();
    loop {
        if instruction_pointer >= computer.instructions.len() - 1 {
            break;
        }
        let instruction = computer.instructions[instruction_pointer];
        let operand = computer.instructions[instruction_pointer + 1];
        match instruction {
            0 => computer.a /= 2_u64.pow(read_combo_operand(&computer, operand) as u32),
            1 => computer.b ^= operand,
            2 => computer.b = read_combo_operand(&computer, operand) % 8,
            3 => {
                if computer.a != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            4 => computer.b ^= computer.c,
            5 => output.push(read_combo_operand(&computer, operand) % 8),
            6 => computer.b = computer.a / 2_u64.pow(read_combo_operand(&computer, operand) as u32),
            7 => computer.c = computer.a / 2_u64.pow(read_combo_operand(&computer, operand) as u32),
            _ => unreachable!(),
        }
        instruction_pointer += 2;
    }
    output
}

fn read_combo_operand(computer: &Computer, operand: u64) -> u64 {
    match operand {
        i if i < 4 => i,
        4 => computer.a,
        5 => computer.b,
        6 => computer.c,
        _ => unreachable!(),
    }
}
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    instructions: Vec<u64>,
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
    fn part1_correct_output_for_test_input1() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day17_part1(&contents);
        assert_eq!(result, String::from("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day17_part1(&contents);
        assert_eq!(result, String::from("2,7,4,7,2,1,7,5,1"));
    }
}
