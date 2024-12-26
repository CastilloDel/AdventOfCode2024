use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day24_part1(&contents);
    println!("Day24 part 1 result: {result}");
}

#[derive(Debug, Clone)]
enum GateType {
    OR,
    XOR,
    AND,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    output: &'a str,
    gate_type: GateType,
}

impl<'a> Gate<'a> {
    fn apply_gate(&'a self, values: &mut HashMap<&'a str, bool>) -> Option<()> {
        let input1 = *values.get(self.input1)?;
        let input2 = *values.get(self.input2)?;

        let output = match self.gate_type {
            GateType::OR => input1 || input2,
            GateType::XOR => input1 ^ input2,
            GateType::AND => input1 && input2,
        };
        values.insert(self.output, output);
        Some(())
    }
}

fn day24_part1(input: &str) -> usize {
    let (_, (initial_values, gates)) = read_input(input).unwrap();
    let values = initial_values.into_iter().collect::<HashMap<&str, bool>>();
    get_computation_output(&gates, values)
}

fn get_computation_output<'a>(
    gates: &'a Vec<Gate<'a>>,
    mut values: HashMap<&'a str, bool>,
) -> usize {
    let mut gates = gates.iter().collect::<Vec<&Gate>>();
    while !gates.is_empty() {
        let mut remaining_gates = Vec::new();
        for gate in gates {
            if let None = gate.apply_gate(&mut values) {
                remaining_gates.push(gate);
            }
        }
        gates = remaining_gates;
    }
    get_variable_value(&values, "z")
}

fn get_variable_value(values: &HashMap<&str, bool>, name: &str) -> usize {
    let mut z_values = values
        .iter()
        .filter(|(k, _)| k.starts_with(name))
        .collect::<Vec<_>>();
    z_values.sort_unstable_by_key(|&(k, _)| k);
    z_values
        .into_iter()
        .map(|(_, v)| v)
        .enumerate()
        .map(|(i, v)| if *v { 1 } else { 0 } << i)
        .sum()
}

fn read_input(input: &str) -> IResult<&str, (Vec<(&str, bool)>, Vec<Gate>)> {
    separated_pair(
        separated_list1(multispace1, read_initial_state),
        multispace1,
        separated_list1(multispace1, read_gate),
    )(input)
}

fn read_initial_state(input: &str) -> IResult<&str, (&str, bool)> {
    separated_pair(alphanumeric1, tag(": "), map(anychar, |c| c == '1'))(input)
}

fn read_gate(input: &str) -> IResult<&str, Gate> {
    map(
        tuple((
            alphanumeric1,
            preceded(tag(" "), read_gate_type),
            preceded(tag(" "), alphanumeric1),
            preceded(tag(" -> "), alphanumeric1),
        )),
        |(input1, gate_type, input2, output)| Gate {
            input1,
            input2,
            output,
            gate_type,
        },
    )(input)
}

fn read_gate_type(input: &str) -> IResult<&str, GateType> {
    alt((
        map(tag("OR"), |_| GateType::OR),
        map(tag("XOR"), |_| GateType::XOR),
        map(tag("AND"), |_| GateType::AND),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day24_part1(&contents);
        assert_eq!(result, 2024);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day24_part1(&contents);
        assert_eq!(result, 56939028423824);
    }
}
