use std::{
    fs,
    ops::{Index, IndexMut},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
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

    fn m(&self) -> usize {
        self.inner.len()
    }

    fn n(&self) -> usize {
        self.inner[0].len()
    }

    fn iter_with_positions(&self) -> MapIterator<T> {
        MapIterator {
            map_ref: &self,
            current_pos: (0, 0),
        }
    }
}

struct MapIterator<'a, T> {
    map_ref: &'a Matrix<T>,
    current_pos: Position,
}

impl<'a, T> Iterator for MapIterator<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.current_pos.1 += 1;
        if self.current_pos.1 == self.map_ref.n() {
            self.current_pos.1 = 0;
            self.current_pos.0 += 1;
        }
        if self.current_pos.0 == self.map_ref.m() {
            return None;
        }
        Some((self.current_pos, &self.map_ref[self.current_pos]))
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
    Box,
    Empty,
    Robot,
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day15_part1(&contents);
    println!("Day15 part 1 result: {result}");
}

fn day15_part1(input: &str) -> usize {
    let (_, (mut matrix, moves)) = read_input(input).unwrap();
    for move_to_execute in moves {
        matrix = execute_move(matrix, move_to_execute);
    }
    matrix
        .iter_with_positions()
        .filter(|(_, value)| **value == Cell::Box)
        .map(|(pos, _)| pos)
        .map(|pos| pos.0 * 100 + pos.1)
        .sum()
}

fn execute_move(mut matrix: Matrix<Cell>, move_to_execute: Direction) -> Matrix<Cell> {
    let (robot_pos, _) = matrix
        .iter_with_positions()
        .find(|(_, cell)| **cell == Cell::Robot)
        .unwrap(); // Safety: Always one robot
    if check_movable(robot_pos, &matrix, move_to_execute) {
        matrix[robot_pos] = Cell::Empty;
        let mut pos = robot_pos;
        let mut last_cell = Cell::Robot;
        loop {
            let next_pos = matrix
                .get_neighbor_in_direction(pos, move_to_execute)
                .unwrap();
            let next_cell = matrix[next_pos];
            matrix[next_pos] = last_cell;
            if next_cell == Cell::Empty {
                break;
            }
            pos = next_pos;
            last_cell = next_cell;
        }
    }
    matrix
}

fn check_movable(robot_pos: Position, matrix: &Matrix<Cell>, move_to_execute: Direction) -> bool {
    let mut pos = robot_pos;
    loop {
        if let Some(next_pos) = matrix.get_neighbor_in_direction(pos, move_to_execute) {
            match matrix[next_pos] {
                Cell::Box => {
                    pos = next_pos;
                }
                Cell::Empty => return true,
                Cell::Wall => return false,
                Cell::Robot => unreachable!(),
            }
        }
    }
}

fn read_input(input: &str) -> IResult<&str, (Matrix<Cell>, Vec<Direction>)> {
    let (input, (matrix, moves)) = separated_pair(read_matrix, multispace1, read_moves)(input)?;
    Ok((input, (Matrix { inner: matrix }, moves)))
}

fn read_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(preceded(multispace0, read_move))(input)
}

fn read_move(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("^"), |_| Direction::Up),
        map(tag(">"), |_| Direction::Right),
        map(tag("v"), |_| Direction::Down),
        map(tag("<"), |_| Direction::Left),
    ))(input)
}

fn read_matrix(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(multispace1, many1(read_cell))(input)
}

fn read_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(tag("#"), |_| Cell::Wall),
        map(tag("."), |_| Cell::Empty),
        map(tag("@"), |_| Cell::Robot),
        map(tag("O"), |_| Cell::Box),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input1() {
        let contents = fs::read_to_string("test_input1").unwrap();
        let result = day15_part1(&contents);
        assert_eq!(result, 2028);
    }

    #[test]
    fn part1_correct_output_for_test_input2() {
        let contents = fs::read_to_string("test_input2").unwrap();
        let result = day15_part1(&contents);
        assert_eq!(result, 10092);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day15_part1(&contents);
        assert_eq!(result, 1509074);
    }
}
