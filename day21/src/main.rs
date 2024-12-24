use std::{collections::HashMap, fs};

/*
Example of equivalent sequences of different lengths


^           ^   <               <               A

<     A     A   <     v   A     A  >   >  ^     A

v<<A  >>^A  A   <vA   <A  >>^A  A  vA  A  <^A   >A

<vA  <A  A   >>^A  A   vA   <^A   >A  A   vA  ^A

v    <   <   A     A   >    ^     A   A   >   A

<                  <              ^   ^   A
*/

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day21_part1(&contents);
    println!("Day21 part 1 result: {result}");
    let result = day21_part2(&contents);
    println!("Day21 part 2 result: {result}");
}

fn day21_part1(input: &str) -> usize {
    let codes = read_input(input);
    let mut memo = HashMap::new();
    let lengths = codes.iter().map(|code| {
        get_moves_for_numeric_keyboard(code)
            .into_iter()
            .map(|code| search(&mut memo, &code, 2))
            .min()
            .unwrap()
    });
    let code_values = codes
        .iter()
        .map(|code| code[0..code.len() - 1].parse::<usize>().unwrap());
    lengths
        .zip(code_values)
        .map(|(sequence, code_value)| sequence * code_value)
        .sum()
}

fn day21_part2(input: &str) -> usize {
    let codes = read_input(input);
    let mut memo = HashMap::new();
    let lengths = codes.iter().map(|code| {
        get_moves_for_numeric_keyboard(code)
            .into_iter()
            .map(|code| search(&mut memo, &code, 25))
            .min()
            .unwrap()
    });
    let code_values = codes
        .iter()
        .map(|code| code[0..code.len() - 1].parse::<usize>().unwrap());
    lengths
        .zip(code_values)
        .map(|(sequence, code_value)| sequence * code_value)
        .sum()
}

fn search(memo: &mut HashMap<(String, usize), usize>, code: &str, times: usize) -> usize {
    if times == 0 {
        return code.len();
    }
    if let Some(result) = memo.get(&(code.to_string(), times)) {
        return *result;
    }
    let result = get_moves_for_directional_keyboard(code)
        .into_iter()
        .map(|sequence| {
            sequence
                .split_inclusive("A")
                .map(|independent_sequence| search(memo, independent_sequence, times - 1))
                .sum()
        })
        .min()
        .unwrap();
    memo.insert((code.to_string(), times), result);
    result
}

type Position = (usize, usize);

const UP: char = '^';
const RIGHT: char = '>';
const DOWN: char = 'v';
const LEFT: char = '<';

fn get_moves_for_numeric_keyboard(code: &str) -> Vec<String> {
    let numerical_keyboard = HashMap::from([
        ('A', (0, 0)),
        ('0', (0, 1)),
        ('3', (1, 0)),
        ('2', (1, 1)),
        ('1', (1, 2)),
        ('6', (2, 0)),
        ('5', (2, 1)),
        ('4', (2, 2)),
        ('9', (3, 0)),
        ('8', (3, 1)),
        ('7', (3, 2)),
    ]);
    get_moves_for_keyboard(code, &numerical_keyboard, (0, 0), (0, 2))
}

fn get_moves_for_directional_keyboard(code: &str) -> Vec<String> {
    let numerical_keyboard = HashMap::from([
        (RIGHT, (0, 0)),
        (DOWN, (0, 1)),
        (LEFT, (0, 2)),
        ('A', (1, 0)),
        (UP, (1, 1)),
    ]);
    get_moves_for_keyboard(code, &numerical_keyboard, (1, 0), (1, 2))
}

fn get_moves_for_keyboard(
    code: &str,
    keyboard: &HashMap<char, Position>,
    start: Position,
    illegal_pos: Position,
) -> Vec<String> {
    let mut result = vec![String::from("")];
    let mut pos = start;
    for c in code.chars() {
        let char_possibilities = &get_moves(pos, keyboard[&c], illegal_pos);
        result = result
            .into_iter()
            .flat_map(|solution| {
                char_possibilities
                    .iter()
                    .map(move |new_part| solution.clone() + new_part)
            })
            .collect();
        pos = keyboard[&c];
    }
    result
}

fn get_moves(start: Position, end: Position, illegal_pos: Position) -> Vec<String> {
    let vertical_diff = start.0 as isize - end.0 as isize;
    let horizontal_diff = start.1 as isize - end.1 as isize;
    let vertical_dir = if vertical_diff > 0 { DOWN } else { UP };
    let horizontal_dir = if horizontal_diff > 0 { RIGHT } else { LEFT };
    let vertical_movement = vertical_dir
        .to_string()
        .repeat(vertical_diff.unsigned_abs());
    let horizontal_movement = horizontal_dir
        .to_string()
        .repeat(horizontal_diff.unsigned_abs());

    if vertical_diff == 0 && horizontal_diff == 0 {
        return vec!["A".to_string()];
    }
    if vertical_diff == 0 {
        return vec![horizontal_movement + "A"];
    }
    if horizontal_diff == 0 {
        return vec![vertical_movement + "A"];
    }

    let mut result = Vec::new();
    if (((start.0 as isize) - vertical_diff) as usize, start.1) != illegal_pos {
        result.push(vertical_movement.clone() + &horizontal_movement + "A");
    }
    if (start.0, ((start.1 as isize) - horizontal_diff) as usize) != illegal_pos {
        result.push(horizontal_movement + &vertical_movement + "A");
    }
    result
}

fn read_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day21_part1(&contents);
        assert_eq!(result, 126384);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day21_part1(&contents);
        assert_eq!(result, 176650);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day21_part2(&contents);
        assert_eq!(result, 154115708116294);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day21_part2(&contents);
        assert_eq!(result, 217698355426872);
    }
}
