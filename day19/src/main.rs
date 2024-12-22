use std::{collections::HashMap, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day19_part1(&contents);
    println!("Day19 part 1 result: {result}");
    let result = day19_part2(&contents);
    println!("Day19 part 2 result: {result}");
}

fn day19_part1(input: &str) -> usize {
    let (_, (available_designs, desired_designs)) = read_input(input).unwrap();
    let available_designs = convert_available_designs_to_dict(available_designs);
    desired_designs
        .into_iter()
        .filter(|design| is_design_possible(design, &available_designs))
        .count()
}

fn day19_part2(input: &str) -> usize {
    let (_, (available_designs, desired_designs)) = read_input(input).unwrap();
    let available_designs = convert_available_designs_to_dict(available_designs);
    desired_designs
        .into_iter()
        .map(|design| count_possible_designs(design, &available_designs, &mut HashMap::new()))
        .sum()
}

fn is_design_possible(design: &str, available_designs: &HashMap<char, Vec<&str>>) -> bool {
    if design.is_empty() {
        return true;
    }
    let first_char = design.chars().nth(0).unwrap(); // Safety: All of the design have at least one char
    for &available_design in available_designs.get(&first_char).unwrap_or(&vec![]) {
        if available_design.len() <= design.len()
            && &design[0..available_design.len()] == available_design
            && is_design_possible(&design[available_design.len()..], available_designs)
        {
            return true;
        }
    }
    false
}

fn count_possible_designs<'a>(
    design: &'a str,
    available_designs: &HashMap<char, Vec<&str>>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }
    let mut total = 0;
    let first_char = design.chars().nth(0).unwrap(); // Safety: All of the design have at least one char
    for &available_design in available_designs.get(&first_char).unwrap_or(&vec![]) {
        if available_design.len() <= design.len()
            && &design[0..available_design.len()] == available_design
        {
            total +=
                count_possible_designs(&design[available_design.len()..], available_designs, memo);
        }
    }
    memo.insert(design, total);
    total
}

fn convert_available_designs_to_dict(available_designs: Vec<&str>) -> HashMap<char, Vec<&str>> {
    let mut dict: HashMap<char, Vec<&str>> = HashMap::new();
    for design in available_designs {
        let first_char = design.chars().nth(0).unwrap(); // Safety: All of the design have at least one char
        dict.entry(first_char)
            .and_modify(|value| value.push(design))
            .or_insert(vec![design]);
    }
    dict
}

fn read_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list1(tag(", "), alpha1),
        multispace1,
        separated_list1(multispace1, alpha1),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day19_part1(&contents);
        assert_eq!(result, 6);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day19_part1(&contents);
        assert_eq!(result, 240);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day19_part2(&contents);
        assert_eq!(result, 16);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day19_part2(&contents);
        assert_eq!(result, 848076019766013);
    }
}
