use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day25_part1(&contents);
    println!("Day25 part 1 result: {result}");
}

fn day25_part1(input: &str) -> usize {
    let (locks, keys) = read_input(input);
    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.iter().zip(lock).map(|(a, b)| a + b).all(|v| v <= 5) {
                count += 1;
            }
        }
    }
    count
}

fn read_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in blocks {
        let mut lines = block.lines();
        let is_lock = lines.next().unwrap() == "#####";
        let mut block_result = vec![0; 5];
        for line in lines.take(5) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    block_result[i] += 1;
                }
            }
        }
        if is_lock {
            locks.push(block_result);
        } else {
            keys.push(block_result);
        }
    }
    (locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day25_part1(&contents);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day25_part1(&contents);
        assert_eq!(result, 3114);
    }
}
