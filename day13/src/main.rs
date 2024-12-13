use nom::{
    character::complete::{multispace1, satisfy, u32},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    AsChar, IResult,
};
use std::fs;

type Vector = (u32, u32);

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day13_part1(&contents);
    println!("Day13 part 1 result: {result}");
}

fn day13_part1(input: &str) -> u32 {
    let (_, claw_machines) = read_input(input).unwrap();
    claw_machines
        .into_iter()
        .filter_map(solve_claw_machine)
        .sum()
}

#[derive(Debug)]
struct ClawMachine {
    a_movement: Vector,
    b_movement: Vector,
    prize: Vector,
}

fn solve_claw_machine(machine: ClawMachine) -> Option<u32> {
    let possible_solutions_first_axis =
        solve_claw_machine_one_axis(machine.a_movement.0, machine.b_movement.0, machine.prize.0);
    let possible_solutions_second_axis =
        solve_claw_machine_one_axis(machine.a_movement.1, machine.b_movement.1, machine.prize.1);
    dbg!(machine);
    dbg!(possible_solutions_first_axis
        .iter()
        .filter(|solution1| {
            possible_solutions_second_axis
                .iter()
                .find(|solution2| solution1 == solution2)
                .is_some()
        })
        .map(|v| v.0 * 3 + v.1)
        .min())
}

fn solve_claw_machine_one_axis(a_movement: u32, b_movement: u32, prize: u32) -> Vec<Vector> {
    let a_limit = prize / a_movement;
    let mut possible_solutions = Vec::new();
    for i in 0..=a_limit {
        let rest = prize - a_movement * i;
        if rest % b_movement == 0 {
            possible_solutions.push((i, rest / b_movement));
        }
    }
    possible_solutions
        .into_iter()
        .filter(|pushes| pushes.0 <= 100 && pushes.1 <= 100)
        .collect()
}

fn read_input(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    separated_list1(multispace1, read_claw_machine)(input)
}

fn read_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, a_movement) = read_vector(input)?;
    let (input, b_movement) = read_vector(input)?;
    let (input, prize) = read_vector(input)?;

    Ok((
        input,
        ClawMachine {
            a_movement,
            b_movement,
            prize,
        },
    ))
}
fn read_vector(input: &str) -> IResult<&str, Vector> {
    tuple((
        preceded(many1(satisfy(|c| !c.is_dec_digit())), u32),
        preceded(many1(satisfy(|c| !c.is_dec_digit())), u32),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day13_part1(&contents);
        assert_eq!(result, 480);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day13_part1(&contents);
        assert_eq!(result, 32067);
    }
}
