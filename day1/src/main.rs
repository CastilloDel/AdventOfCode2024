use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day1_part1(&contents);
    println!("Day1 part 1 result: {result}");
    let result = day1_part2(&contents);
    println!("Day1 part 2 result: {result}")
}

fn day1_part1(input: &str) -> i64 {
    let (mut numbers1, mut numbers2) = read_number_lists(input);
    numbers1.sort_unstable();
    numbers2.sort_unstable();
    numbers1
        .into_iter()
        .zip(numbers2)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn day1_part2(input: &str) -> i64 {
    let (numbers1, numbers2) = read_number_lists(input);
    let mut occurrences = HashMap::new();
    for n in numbers2 {
        *occurrences.entry(n).or_insert(0) += 1;
    }
    numbers1
        .into_iter()
        .map(|n| n * occurrences.get(&n).unwrap_or(&0))
        .sum()
}

fn read_number_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut numbers1 = Vec::new();
    let mut numbers2 = Vec::new();
    for line in input.lines() {
        let line_numbers = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        numbers1.push(line_numbers[0]);
        numbers2.push(line_numbers[1]);
    }
    (numbers1, numbers2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 11);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 2970687);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day1_part2(&contents);
        assert_eq!(result, 31);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day1_part2(&contents);
        assert_eq!(result, 23963899);
    }
}
