use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day1_part1(&contents);
    println!("Day1 part 1 result: {result}");
}

fn day1_part1(input: &str) -> i64 {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|s| {
            let (_, [num1, num2]) = s.extract();
            let num1 = num1.parse::<i64>().unwrap();
            let num2 = num2.parse::<i64>().unwrap();
            num1 * num2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 161);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 178886550);
    }
}
