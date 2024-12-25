use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day22_part1(&contents);
    println!("Day22 part 1 result: {result}");
    let result = day22_part2(&contents);
    println!("Day22 part 2 result: {result}");
}

fn day22_part1(input: &str) -> u64 {
    let mut initial_numbers = read_input(input);
    for _ in 0..2000 {
        initial_numbers = initial_numbers.into_iter().map(apply_evolution).collect();
    }
    initial_numbers.into_iter().sum()
}

fn day22_part2(input: &str) -> i64 {
    let mut secret_numbers = read_input(input);
    let mut differences_with_reward = vec![Vec::with_capacity(2000); secret_numbers.len()];
    for _ in 0..2000 {
        let new_numbers = secret_numbers
            .iter()
            .copied()
            .map(apply_evolution)
            .collect::<Vec<_>>();
        let new_differences_with_reward = new_numbers
            .iter()
            .zip(secret_numbers)
            .map(|(&a, b)| {
                let reward = (a % 10) as i64;
                let old_reward = (b % 10) as i64;
                (reward - old_reward, reward)
            })
            .collect::<Vec<_>>();
        for (i, diff_with_reward) in new_differences_with_reward.into_iter().enumerate() {
            differences_with_reward[i].push(diff_with_reward);
        }
        secret_numbers = new_numbers;
    }
    let windows_with_rewards = differences_with_reward
        .into_iter()
        .flat_map(|a| {
            a.windows(4)
                .map(|window| {
                    let diff_window = window
                        .iter()
                        .map(|(diff, _)| diff)
                        .copied()
                        .collect::<Vec<i64>>();
                    (diff_window, window[3].1)
                })
                .fold(HashMap::new(), |mut acc, (window, reward)| {
                    if !acc.contains_key(&window) {
                        acc.insert(window, reward);
                    }
                    acc
                })
        })
        .fold(HashMap::new(), |mut acc, (window, reward)| {
            let value = reward + acc.get(&window).unwrap_or(&0);
            acc.insert(window, value);
            acc
        });
    *windows_with_rewards.values().max().unwrap()
}

fn apply_evolution(mut n: u64) -> u64 {
    n ^= n << 6;
    n &= 0xffffff;
    n ^= n >> 5;
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
        let contents = fs::read_to_string("test_input_part1").unwrap();
        let result = day22_part1(&contents);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day22_part1(&contents);
        assert_eq!(result, 15613157363);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input_part2").unwrap();
        let result = day22_part2(&contents);
        assert_eq!(result, 23);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day22_part2(&contents);
        assert_eq!(result, 1784);
    }
}
