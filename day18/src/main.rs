use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Index, IndexMut},
};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Position = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone)]
struct Matrix<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn get_neighbor_in_direction(&self, pos: Position, direction: Direction) -> Option<Position> {
        let change = match direction {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
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

    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        directions
            .into_iter()
            .filter_map(move |dir| self.get_neighbor_in_direction(pos, dir))
            .collect()
    }

    fn m(&self) -> usize {
        self.inner.len()
    }

    fn n(&self) -> usize {
        self.inner[0].len()
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.inner[pos.0][pos.1]
    }
}

impl<T> IndexMut<Position> for Matrix<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.inner[pos.0][pos.1]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
}

fn main() {
    let contents = fs::read_to_string("message.txt").unwrap();
    let result = day18_part1(&contents, 71, 1024);
    println!("Day18 part 1 result: {result}");
    let result = day18_part2(&contents, 71, 1024);
    println!("Day18 part 1 result: {result}");
}

fn day18_part1(input: &str, size: usize, fallen_bytes: usize) -> usize {
    let (_, bytes) = read_input(input).unwrap();
    let mut matrix = Matrix {
        inner: vec![vec![Cell::Empty; size]; size],
    };
    for byte_pos in bytes.into_iter().take(fallen_bytes) {
        matrix[byte_pos] = Cell::Wall;
    }
    get_distance_to_end(&matrix, (0, 0), (size - 1, size - 1)).unwrap()
}

fn day18_part2(input: &str, size: usize, fallen_bytes: usize) -> String {
    let (_, bytes) = read_input(input).unwrap();
    let mut matrix = Matrix {
        inner: vec![vec![Cell::Empty; size]; size],
    };
    for &byte_pos in bytes.iter().take(fallen_bytes) {
        matrix[byte_pos] = Cell::Wall;
    }
    for &byte_pos in bytes.iter().skip(fallen_bytes) {
        matrix[byte_pos] = Cell::Wall;

        if get_distance_to_end(&matrix, (0, 0), (size - 1, size - 1)).is_none() {
            return format!("{},{}", byte_pos.1, byte_pos.0);
        }
    }
    unreachable!()
}

fn guess_distance(pos1: Position, pos2: Position) -> usize {
    (pos1.0 as isize - pos2.0 as isize).abs() as usize
        + (pos1.1 as isize - pos2.1 as isize).abs() as usize
}

fn get_distance_to_end(matrix: &Matrix<Cell>, start: Position, end: Position) -> Option<usize> {
    let mut open = HashSet::from([start]);

    let mut costs = HashMap::new();
    costs.insert(start, 0);
    let mut guesses = HashMap::new();
    guesses.insert(start, guess_distance(start, end));

    while !open.is_empty() {
        let current = open.iter().min_by_key(|pos| guesses[pos]).unwrap().clone();
        open.remove(&current);
        if current == end {
            return Some(costs[&current]);
        }

        let neighbors = matrix
            .get_neighbors(current)
            .into_iter()
            .filter(|&pos| matrix[pos] != Cell::Wall)
            .collect::<Vec<_>>();
        for neighbor in neighbors {
            let tentative_cost = costs[&current] + 1;
            if tentative_cost < *costs.get(&neighbor).unwrap_or(&usize::MAX) {
                costs.insert(neighbor, tentative_cost);
                guesses.insert(neighbor, tentative_cost + guess_distance(neighbor, end));
                open.insert(neighbor);
            }
        }
    }

    None
}

fn read_input(input: &str) -> IResult<&str, Vec<Position>> {
    separated_list1(multispace1, read_position)(input)
}

fn read_position(input: &str) -> IResult<&str, Position> {
    map(separated_pair(u64, tag(","), u64), |(x, y)| {
        (y as usize, x as usize) // Swap the order as Matrix uses (i,j) instead of (x,y) (row first)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day18_part1(&contents, 7, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day18_part1(&contents, 71, 1024);
        assert_eq!(result, 506);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day18_part2(&contents, 7, 12);
        assert_eq!(result, String::from("6,1"));
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day18_part2(&contents, 71, 1024);
        assert_eq!(result, String::from("62,6"));
    }
}
