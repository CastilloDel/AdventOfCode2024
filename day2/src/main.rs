use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day2_part1(&contents);
    println!("Day1 part 1 result: {result}");
    let result = day2_part2(&contents);
    println!("Day1 part 2 result: {result}");
}

fn day2_part1(input: &str) -> usize {
    let numbers = read_number_lists(input);
    numbers
        .iter()
        .map(check_report)
        .filter(|&val| val)
        .collect::<Vec<_>>()
        .len()
}

fn day2_part2(input: &str) -> usize {
    let numbers = read_number_lists(input);
    numbers
        .iter()
        .map(check_report_with_dampener)
        .filter(|&val| val)
        .count()
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

fn check_report_with_dampener(report: &Vec<i64>) -> bool {
    (0..report.len()).any(|i| {
        let mut report_clone = report.clone();
        report_clone.remove(i);
        check_report(&report_clone)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day2_part1(&contents);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day2_part1(&contents);
        assert_eq!(result, 490);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day2_part2(&contents);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day2_part2(&contents);
        assert_eq!(result, 536);
    }
}
