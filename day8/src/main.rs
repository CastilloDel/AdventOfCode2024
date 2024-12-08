use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Position = (isize, isize);

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day8_part1(&contents);
    println!("Day8 part 1 result: {result}");
}

fn day8_part1(input: &str) -> usize {
    let antennas = read_antennas(input);
    let m = input.lines().count();
    let n = input.lines().next().unwrap().len();
    antennas
        .into_iter()
        .flat_map(|(_, antennas)| get_antinodes_for_type(&antennas, m, n))
        .collect::<HashSet<_>>()
        .len()
}

fn get_antinodes_for_type(antennas: &[Position], m: usize, n: usize) -> Vec<Position> {
    antennas
        .iter()
        .enumerate()
        .flat_map(|(index, &pos1)| antennas[index + 1..].iter().map(move |&pos2| (pos1, pos2)))
        .flat_map(|(pos1, pos2)| get_number_of_antinodes_for_pair(pos1, pos2, m, n))
        .collect()
}

fn get_number_of_antinodes_for_pair(
    pos1: Position,
    pos2: Position,
    m: usize,
    n: usize,
) -> Vec<Position> {
    let distance = (pos1.0 - pos2.0, pos1.1 - pos2.1);
    let antinode1 = (pos1.0 + distance.0, pos1.1 + distance.1);
    let antinode2 = (pos2.0 - distance.0, pos2.1 - distance.1);
    [antinode1, antinode2]
        .into_iter()
        .filter(|&pos| check_bounds(pos, m, n))
        .collect()
}

fn check_bounds(pos: Position, m: usize, n: usize) -> bool {
    pos.0 >= 0 && pos.0 < m as isize && pos.1 >= 0 && pos.1 < n as isize
}

fn read_antennas(input: &str) -> HashMap<char, Vec<Position>> {
    let mut antennas = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let entry = antennas.entry(c).or_insert(Vec::new());
            entry.push((i as isize, j as isize));
        }
    }
    antennas
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day8_part1(&contents);
        assert_eq!(result, 14);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day8_part1(&contents);
        assert_eq!(result, 2613);
    }
}
