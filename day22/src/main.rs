use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day22_part1(&contents);
    println!("Day22 part 1 result: {result}");
}

fn day22_part1(input: &str) -> u64 {
    let mut initial_numbers = read_input(input);
    for _ in 0..2000 {
        initial_numbers = initial_numbers.into_iter().map(apply_evolution).collect();
    }
    initial_numbers.into_iter().sum()
}

fn apply_evolution(mut n: u64) -> u64 {
    n ^= n << 6;
    n &= 0xffffff;
    n ^= n >> 5;
    n &= 0xffffff;
    n ^= n << 11;
    n &= 0xffffff;
    n
}

fn read_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day22_part1(&contents);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day22_part1(&contents);
        assert_eq!(result, 15613157363);
    }
}
