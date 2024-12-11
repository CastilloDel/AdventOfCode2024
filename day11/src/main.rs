use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day11_part1(&contents);
    println!("Day11 part 1 result: {result}");
}

fn day11_part1(input: &str) -> usize {
    let mut stones = read_input(input);
    for _ in 0..25 {
        stones = apply_blink_to_stones(stones);
    }
    stones.len()
}

fn apply_blink_to_stones(stones: Vec<Stone>) -> Vec<Stone> {
    stones
        .into_iter()
        .flat_map(|stone| apply_blink_to_stone(stone))
        .collect()
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
}
