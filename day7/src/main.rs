use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day7_part1(&contents);
    println!("Day7 part 1 result: {result}");
    let result = day7_part2(&contents);
    println!("Day7 part 2 result: {result}");
}

fn day7_part1(input: &str) -> u64 {
    let (_, equations) = read_input(input).unwrap();
    let allowed_operations = [Operation::Addition, Operation::Multiplication];
    equations
        .into_iter()
        .filter(|equation| check_equation(equation, &allowed_operations))
        .map(|equation| equation.test_value)
        .sum()
}

fn day7_part2(input: &str) -> u64 {
    let (_, equations) = read_input(input).unwrap();
    let allowed_operations = [
        Operation::Addition,
        Operation::Multiplication,
        Operation::Concatenation,
    ];
    equations
        .into_iter()
        .filter(|equation| check_equation(equation, &allowed_operations))
        .map(|equation| equation.test_value)
        .sum()
}

fn check_equation(equation: &Equation, operations: &[Operation]) -> bool {
    if equation.numbers.len() == 1 {
        return equation.test_value == equation.numbers[0];
    }
    operations
        .iter()
        .any(|op| check_equation(&apply_operation(equation.clone(), op), operations))
}

fn apply_operation(mut equation: Equation, operation: &Operation) -> Equation {
    let first_number = equation.numbers.remove(0);
    equation.numbers[0] = match operation {
        Operation::Addition => first_number + equation.numbers[0],
        Operation::Multiplication => first_number * equation.numbers[0],
        Operation::Concatenation => format!("{}{}", first_number, equation.numbers[0])
            .parse()
            .unwrap(),
    };
    equation
}

fn read_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(multispace1, read_equation)(input)
}

fn read_equation(input: &str) -> IResult<&str, Equation> {
    let (input, (test_value, numbers)) =
        separated_pair(u64, tag(": "), separated_list1(tag(" "), u64))(input)?;

    Ok((
        input,
        Equation {
            test_value,
            numbers,
        },
    ))
}

#[derive(Clone)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

#[derive(Clone, Copy)]
enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day7_part1(&contents);
        assert_eq!(result, 3749);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day7_part1(&contents);
        assert_eq!(result, 28730327770375);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day7_part2(&contents);
        assert_eq!(result, 11387);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day7_part2(&contents);
        assert_eq!(result, 424977609625985);
    }
}
