use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day11_part1(&contents);
    println!("Day11 part 1 result: {result}");
    let result = day11_part2(&contents);
    println!("Day11 part 2 result: {result}");
}

fn day11_part1(input: &str) -> usize {
    let stones = read_input(input);
    apply_blinks_to_stones(stones, 25)
}

fn day11_part2(input: &str) -> usize {
    let stones = read_input(input);
    apply_blinks_to_stones(stones, 75)
}

fn apply_blinks_to_stones(stones: Vec<Stone>, times: usize) -> usize {
    let mut memo = HashMap::new();
    stones
        .into_iter()
        .map(|stone| apply_blinks_to_stone(stone, times, &mut memo))
        .sum()
}

fn apply_blinks_to_stone(
    stone: Stone,
    times: usize,
    memo: &mut HashMap<(Stone, usize), usize>,
) -> usize {
    if times == 0 {
        return 1;
    }
    if memo.contains_key(&(stone, times)) {
        return memo[&(stone, times)];
    }
    let stones = apply_blink_to_stone(stone)
        .into_iter()
        .map(|stone| apply_blinks_to_stone(stone, times - 1, memo))
        .sum();
    memo.insert((stone, times), stones);
    stones
}

fn apply_blink_to_stone(stone: Stone) -> Vec<Stone> {
    let stone_str = stone.to_string();
    match stone {
        0 => vec![1],
        _ if stone_str.len() % 2 == 0 => {
            vec![
                stone_str[..stone_str.len() / 2].parse().unwrap(),
                stone_str[stone_str.len() / 2..].parse().unwrap(),
            ]
        }
        _ => vec![stone * 2024],
    }
}

type Stone = usize;

fn read_input(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect::<Vec<Stone>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day11_part1(&contents);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day11_part1(&contents);
        assert_eq!(result, 189092);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day11_part2(&contents);
        assert_eq!(result, 65601038650482);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day11_part2(&contents);
        assert_eq!(result, 224869647102559);
    }
}
