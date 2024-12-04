use std::{fs, ops::Index};

type Position = (usize, usize);

#[derive(Copy, Clone)]
enum Direction {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
}

struct Matrix {
    inner: Vec<Vec<char>>,
}

impl Matrix {
    fn get_neighbor_in_direction(&self, pos: Position, direction: Direction) -> Option<Position> {
        let change = match direction {
            Direction::Up => (1, 0),
            Direction::UpLeft => (1, -1),
            Direction::Left => (0, -1),
            Direction::DownLeft => (-1, -1),
            Direction::Down => (-1, 0),
            Direction::DownRight => (-1, 1),
            Direction::Right => (0, 1),
            Direction::UpRight => (1, 1),
        };
        if pos.0 == 0 && change.0 == -1
            || pos.1 == 0 && change.1 == -1
            || pos.0 == self.m() - 1 && change.0 == 1
            || pos.1 == self.n() - 1 && change.1 == 1
        {
            return None;
        }
        Some((
            (pos.0 as isize + change.0) as usize,
            (pos.1 as isize + change.1) as usize,
        ))
    }

    fn m(&self) -> usize {
        self.inner.len()
    }

    fn n(&self) -> usize {
        self.inner[0].len()
    }
}

impl Index<Position> for Matrix {
    type Output = char;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.inner[pos.0][pos.1]
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day1_part1(&contents);
    println!("Day1 part 1 result: {result}");
}

fn day1_part1(input: &str) -> usize {
    let matrix = read_letter_matrix(input);
    (0..matrix.m())
        .flat_map(|i| (0..matrix.n()).map(move |j| (i, j)))
        .map(|(i, j)| find_xmas_in_neighborhood(&matrix, (i, j)))
        .sum()
}

fn find_xmas_in_neighborhood(matrix: &Matrix, pos: Position) -> usize {
    [
        Direction::Up,
        Direction::UpLeft,
        Direction::Left,
        Direction::DownLeft,
        Direction::Down,
        Direction::DownRight,
        Direction::Right,
        Direction::UpRight,
    ]
    .iter()
    .zip(std::iter::repeat(pos))
    .map(|(direction, pos)| find_xmas_in_direction(matrix, pos, *direction))
    .filter(|&condition| condition)
    .count()
}

fn find_xmas_in_direction(matrix: &Matrix, mut pos: Position, direction: Direction) -> bool {
    if matrix[pos] != 'X' {
        return false;
    }
    for letter in "MAS".chars() {
        pos = match matrix.get_neighbor_in_direction(pos, direction) {
            Some(pos) => pos,
            None => return false,
        };
        if matrix[pos] != letter {
            return false;
        }
    }
    true
}

fn read_letter_matrix(input: &str) -> Matrix {
    Matrix {
        inner: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 18);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day1_part1(&contents);
        assert_eq!(result, 2613);
    }
}
