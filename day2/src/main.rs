use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day1_part1(&contents);
    println!("Day1 part 1 result: {result}");
}

fn day1_part1(input: &str) -> usize {
    let numbers = read_number_lists(input);
    numbers
        .iter()
        .map(check_report)
        .filter(|&val| val)
        .collect::<Vec<_>>()
        .len()
}

fn read_number_lists(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn check_report(report: &Vec<i64>) -> bool {
    let differences = report
        .windows(2)
        .map(|window| window[0] - window[1])
        .collect::<Vec<_>>();
    let ascending = differences.iter().all(|&n| n < 0 && n > -4);
    let descending = differences.iter().all(|&n| n > 0 && n < 4);
    ascending || descending
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 490);
    }
}
