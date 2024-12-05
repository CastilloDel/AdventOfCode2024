use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day5_part1(&contents);
    println!("Day5 part 1 result: {result}");
    let result = day5_part2(&contents);
    println!("Day5 part 2 result: {result}");
}

fn day5_part1(input: &str) -> u32 {
    let (_, (rules, updates)) = read_input(input).unwrap();
    let rules = convert_rules_into_hashmap(rules);
    updates
        .into_iter()
        .filter(|update| is_update_safe(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn day5_part2(input: &str) -> u32 {
    let (_, (rules, updates)) = read_input(input).unwrap();
    let rules = convert_rules_into_hashmap(rules);
    updates
        .into_iter()
        .filter(|update| !is_update_safe(update, &rules))
        .map(|update| fix_update(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn convert_rules_into_hashmap(rules: Vec<Rule>) -> HashMap<u32, HashSet<u32>> {
    rules.into_iter().fold(HashMap::new(), |mut map, rule| {
        map.entry(rule.left)
            .and_modify(|rights: &mut HashSet<_>| {
                rights.insert(rule.right);
            })
            .or_insert(HashSet::from([rule.right]));
        map
    })
}

fn is_update_safe(update: &Update, rules: &HashMap<u32, HashSet<u32>>) -> bool {
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

fn fix_update(mut update: Update, rules: &HashMap<u32, HashSet<u32>>) -> Update {
    let mut index = 0;
    while index < update.len() {
        let number = &update[index];
        if let Some(rights) = rules.get(number) {
            let offending_index = update[..index]
                .iter()
                .position(|left| rights.iter().any(|right| left == right));
            if let Some(offending_index) = offending_index {
                update.swap(index, offending_index);
                index = offending_index;
                continue;
            }
        }
        index += 1;
    }
    update
}

fn read_input(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Update>)> {
    separated_pair(
        separated_list1(multispace1, read_rule),
        multispace1,
        separated_list1(multispace1, read_update),
    )(input)
}

fn read_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (left, right)) = separated_pair(u32, tag("|"), u32)(input)?;

    Ok((input, Rule { left, right }))
}

fn read_update(input: &str) -> IResult<&str, Update> {
    separated_list1(tag(","), u32)(input)
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

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day5_part2(&contents);
        assert_eq!(result, 123);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day5_part2(&contents);
        assert_eq!(result, 5184);
    }
}
