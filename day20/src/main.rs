use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Index, IndexMut},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::map,
    multi::{many1, separated_list1},
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

    fn iter_with_positions(&self) -> MapIterator<T> {
        MapIterator {
            map_ref: self,
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
    Empty,
    Start,
    End,
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day20_part1(&contents);
    println!("Day20 part 1 result: {result}");
}

fn day20_part1(input: &str) -> usize {
    let (_, matrix) = read_input(input).unwrap();
    let path = get_path_to_end(&matrix);
    get_cheats(path)
        .iter()
        .filter(|(_, cost)| cost >= &100)
        .count()
}

fn get_cheats(path: Vec<(usize, usize)>) -> Vec<((Position, Position), usize)> {
    let mut cheats = Vec::new();
    for (i, &pos1) in path.iter().enumerate() {
        for (j, &pos2) in path.iter().enumerate().skip(i + 4) {
            if is_cheat_possible(pos1, pos2) {
                cheats.push(((pos1, pos2), j - i - 2));
            }
        }
    }
    cheats
}
fn is_cheat_possible(pos1: Position, pos2: Position) -> bool {
    let vertical_diff = (pos1.0 as isize - pos2.0 as isize).abs();
    let horizontal_diff = (pos1.1 as isize - pos2.1 as isize).abs();
    (vertical_diff == 2 && horizontal_diff == 0) || (horizontal_diff == 2 && vertical_diff == 0)
}

fn guess_distance(pos1: Position, pos2: Position) -> usize {
    (pos1.0 as isize - pos2.0 as isize).unsigned_abs()
        + (pos1.1 as isize - pos2.1 as isize).unsigned_abs()
}

fn get_path_to_end(matrix: &Matrix<Cell>) -> Vec<Position> {
    let (start, _) = matrix
        .iter_with_positions()
        .find(|(_, cell)| **cell == Cell::Start)
        .unwrap(); // Safety: Always one start
    let (end, _) = matrix
        .iter_with_positions()
        .find(|(_, cell)| **cell == Cell::End)
        .unwrap(); // Safety: Always one end

    let mut open = HashSet::from([start]);

    let mut costs = HashMap::new();
    costs.insert(start, 0);
    let mut guesses = HashMap::new();
    guesses.insert(start, guess_distance(start, end));
    let mut came_from = HashMap::new();

    while !open.is_empty() {
        let current = *open.iter().min_by_key(|pos| guesses[pos]).unwrap();
        open.remove(&current);
        if current == end {
            return get_path(current, &came_from);
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
                came_from.insert(neighbor, current);
            }
        }
    }

    unreachable!()
}

fn get_path(end: Position, came_from: &HashMap<Position, Position>) -> Vec<Position> {
    let mut path = Vec::new();
    if came_from.contains_key(&end) {
        path = get_path(came_from[&end], came_from);
    }
    path.insert(0, end);
    path
}

fn read_input(input: &str) -> IResult<&str, Matrix<Cell>> {
    map(read_matrix, |inner| Matrix { inner })(input)
}

fn read_matrix(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(multispace1, many1(read_cell))(input)
}

fn read_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(tag("#"), |_| Cell::Wall),
        map(tag("."), |_| Cell::Empty),
        map(tag("S"), |_| Cell::Start),
        map(tag("E"), |_| Cell::End),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day20_part1(&contents);
        assert_eq!(result, 1507);
    }
}
