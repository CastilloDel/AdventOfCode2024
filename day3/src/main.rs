use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day3_part1(&contents);
    println!("Day1 part 1 result: {result}");
    let result = day3_part2(&contents);
    println!("Day1 part 2 result: {result}");
}

fn day3_part1(input: &str) -> u64 {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|s| {
            let (_, [num1, num2]) = s.extract();
            let num1 = num1.parse::<u64>().unwrap();
            let num2 = num2.parse::<u64>().unwrap();
            num1 * num2
        })
        .sum()
}

fn day3_part2(input: &str) -> u64 {
    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    re.find_iter(input)
        .map(|instruction| instruction.as_str())
        .filter_map(|instruction| {
            if instruction == "do()" {
                enabled = true;
            } else if instruction == "don't()" {
                enabled = false;
            } else if enabled {
                let comma_index = instruction.find(",").unwrap();
                let num1 = &instruction[4..comma_index];
                let num2 = &instruction[comma_index + 1..instruction.len() - 1];
                let num1 = num1.parse::<u64>().unwrap();
                let num2 = num2.parse::<u64>().unwrap();
                return Some(num1 * num2);
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input_part1").unwrap();
        let result = day3_part1(&contents);
        assert_eq!(result, 161);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day3_part1(&contents);
        assert_eq!(result, 178886550);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input_part2").unwrap();
        let result = day3_part2(&contents);
        assert_eq!(result, 48);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day3_part2(&contents);
        assert_eq!(result, 87163705);
    }
}
