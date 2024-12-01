use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day1_part1(contents);
    println!("Day1 part 1 result: {result}")
}

fn day1_part1(input: String) -> i64 {
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
    numbers1.sort_unstable();
    numbers2.sort_unstable();
    numbers1
        .into_iter()
        .zip(numbers2.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day1_part1(contents);
        assert_eq!(result, 11);
    }
}
