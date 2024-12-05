use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day5_part1(&contents);
    println!("Day5 part 1 result: {result}");
}

fn day5_part1(input: &str) -> u32 {
    let (_, (rules, updates)) = read_input(input).unwrap();
    let rules = rules.into_iter().fold(HashMap::new(), |mut map, rule| {
        map.entry(rule.left)
            .and_modify(|rights: &mut Vec<_>| rights.push(rule.right))
            .or_insert(vec![rule.right]);
        map
    });
    updates
        .into_iter()
        .filter(|update| is_update_safe(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_update_safe(update: &Update, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut set = HashSet::new();
    set.insert(&update[0]);
    update[1..].iter().all(|number| {
        if let Some(rights) = rules.get(number) {
            if rights.iter().any(|right| set.contains(right)) {
                return false;
            }
        }
        set.insert(number);
        true
    })
}

fn read_input(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Update>)> {
    let (input, rules) = separated_list1(tag("\n"), read_rule)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, updates) = separated_list1(tag("\n"), read_update)(input)?;
    Ok((input, (rules, updates)))
}

fn read_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (left, right)): (&str, (&str, &str)) =
        separated_pair(digit1, tag("|"), digit1)(input)?;

    let left = left.parse::<u32>().unwrap();
    let right = right.parse::<u32>().unwrap();

    Ok((input, Rule { left, right }))
}

fn read_update(input: &str) -> IResult<&str, Update> {
    let (input, numbers) = separated_list1(tag(","), digit1)(input)?;

    let numbers = numbers.iter().map(|n| n.parse::<u32>().unwrap()).collect();

    Ok((input, numbers))
}

struct Rule {
    left: u32,
    right: u32,
}

type Update = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day5_part1(&contents);
        assert_eq!(result, 143);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day5_part1(&contents);
        assert_eq!(result, 6267);
    }
}
