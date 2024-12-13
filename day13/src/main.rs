use nom::{
    character::complete::{multispace1, satisfy, u64},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    AsChar, IResult,
};
use std::fs;

type Vector = (u64, u64);

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day13_part1(&contents);
    println!("Day13 part 1 result: {result}");
    let result = day13_part2(&contents);
    println!("Day13 part 2 result: {result}");
}

fn day13_part1(input: &str) -> u64 {
    let (_, claw_machines) = read_input(input).unwrap();
    claw_machines
        .into_iter()
        .filter_map(solve_claw_machine)
        .sum()
}

fn day13_part2(input: &str) -> u64 {
    let (_, claw_machines) = read_input(input).unwrap();
    claw_machines
        .into_iter()
        .map(adjust_prize)
        .filter_map(solve_claw_machine)
        .sum()
}

#[derive(Debug)]
struct ClawMachine {
    a_movement: Vector,
    b_movement: Vector,
    prize: Vector,
}

fn solve_claw_machine(machine: ClawMachine) -> Option<u64> {
    let denominator_determinant = ((machine.a_movement.0 * machine.b_movement.1) as i64
        - (machine.b_movement.0 * machine.a_movement.1) as i64)
        as f64;
    let x_determinant = ((machine.prize.0 * machine.b_movement.1) as i64
        - (machine.b_movement.0 * machine.prize.1) as i64) as f64;
    let y_determinant = ((machine.a_movement.0 * machine.prize.1) as i64
        - (machine.prize.0 * machine.a_movement.1) as i64) as f64;
    if denominator_determinant == 0.0 {
        return None;
    }

    let a = x_determinant / denominator_determinant;
    let b = y_determinant / denominator_determinant;
    if a.fract() != 0.0 || b.fract() != 0.0 {
        return None;
    }

    Some(a as u64 * 3 + b as u64)
}

fn adjust_prize(mut claw_machine: ClawMachine) -> ClawMachine {
    claw_machine.prize.0 += 10000000000000;
    claw_machine.prize.1 += 10000000000000;
    claw_machine
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
        preceded(many1(satisfy(|c| !c.is_dec_digit())), u64),
        preceded(many1(satisfy(|c| !c.is_dec_digit())), u64),
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

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day13_part2(&contents);
        assert_eq!(result, 875318608908);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day13_part2(&contents);
        assert_eq!(result, 92871736253789);
    }
}
