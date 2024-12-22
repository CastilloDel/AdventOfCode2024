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
    Empty,
    Start,
    End,
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day16_part1(&contents);
    println!("Day16 part 1 result: {result}");
    let result = day16_part2(&contents);
    println!("Day16 part 2 result: {result}");
}

fn day16_part1(input: &str) -> usize {
    let (_, matrix) = read_input(input).unwrap();
    get_distance_to_end(&matrix)
}

fn day16_part2(input: &str) -> usize {
    let (_, matrix) = read_input(input).unwrap();
    get_cells_in_optimal_paths_to_end(&matrix)
}

fn guess_distance(pos1: Position, pos2: Position) -> usize {
    // Better heuristics are possible
    (pos1.0 as isize - pos2.0 as isize).abs() as usize
        + (pos1.1 as isize - pos2.1 as isize).abs() as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PositionWithDirection {
    position: Position,
    direction: Direction,
}

fn get_distance_to_end(matrix: &Matrix<Cell>) -> usize {
    let (start_pos, _) = matrix
        .iter_with_positions()
        .find(|(_, cell)| **cell == Cell::Start)
        .unwrap(); // Safety: Always one start
    let start_direction = Direction::Right;
    let (end_pos, _) = matrix
        .iter_with_positions()
        .find(|(_, cell)| **cell == Cell::End)
        .unwrap(); // Safety: Always one end

    let start = PositionWithDirection {
        position: start_pos,
        direction: start_direction,
    };
    let mut open = HashSet::from([start]);

    let mut costs = HashMap::new();
    costs.insert(start, 0);
    let mut guesses = HashMap::new();
    guesses.insert(start, guess_distance(start_pos, end_pos));

    while !open.is_empty() {
        let current = open.iter().min_by_key(|pos| guesses[pos]).unwrap().clone();
        open.remove(&current);
        if current.position == end_pos {
            return costs[&current];
        }

        let mut neighbors_with_cost = get_neighboring_directions(current.direction)
            .into_iter()
            .map(|dir| PositionWithDirection {
                position: current.position,
                direction: dir,
            })
            .map(|pos| (pos, 1000))
            .collect::<Vec<_>>();
        if let Some(neighbor_pos) =
            matrix.get_neighbor_in_direction(current.position, current.direction)
        {
            if matrix[neighbor_pos] != Cell::Wall {
                let pos = PositionWithDirection {
                    position: neighbor_pos,
                    direction: current.direction,
                };
                neighbors_with_cost.push((pos, 1));
            }
        }
        for (neighbor, cost) in neighbors_with_cost {
            let tentative_cost = costs[&current] + cost;
            if tentative_cost < *costs.get(&neighbor).unwrap_or(&usize::MAX) {
                costs.insert(neighbor, tentative_cost);
                guesses.insert(
                    neighbor,
                    tentative_cost + guess_distance(neighbor.position, end_pos),
                );
                open.insert(neighbor);
            }
        }
    }

    unreachable!()
}

// A bit of duplication between days never hurt anybody XP
fn get_cells_in_optimal_paths_to_end(matrix: &Matrix<Cell>) -> usize {
    let (start_pos, _) = matrix
        .iter_with_positions()
        .find(|(_, cell)| **cell == Cell::Start)
        .unwrap(); // Safety: Always one start
    let start_direction = Direction::Right;
    let (end_pos, _) = matrix
        .iter_with_positions()
        .find(|(_, cell)| **cell == Cell::End)
        .unwrap(); // Safety: Always one end

    let start = PositionWithDirection {
        position: start_pos,
        direction: start_direction,
    };
    let mut open = HashSet::from([start]);

    let mut costs = HashMap::new();
    costs.insert(start, 0);
    let mut guesses = HashMap::new();
    guesses.insert(start, guess_distance(start_pos, end_pos));
    let mut came_from = HashMap::new();

    while !open.is_empty() {
        let current = open.iter().min_by_key(|pos| guesses[pos]).unwrap().clone();
        open.remove(&current);
        if current.position == end_pos {
            return get_cells_in_path(current, &came_from).len();
        }

        let mut neighbors_with_cost = get_neighboring_directions(current.direction)
            .into_iter()
            .map(|dir| PositionWithDirection {
                position: current.position,
                direction: dir,
            })
            .map(|pos| (pos, 1000))
            .collect::<Vec<_>>();
        if let Some(neighbor_pos) =
            matrix.get_neighbor_in_direction(current.position, current.direction)
        {
            if matrix[neighbor_pos] != Cell::Wall {
                let pos = PositionWithDirection {
                    position: neighbor_pos,
                    direction: current.direction,
                };
                neighbors_with_cost.push((pos, 1));
            }
        }
        for (neighbor, cost) in neighbors_with_cost {
            let tentative_cost = costs[&current] + cost;
            if tentative_cost < *costs.get(&neighbor).unwrap_or(&usize::MAX) {
                costs.insert(neighbor, tentative_cost);
                guesses.insert(
                    neighbor,
                    tentative_cost + guess_distance(neighbor.position, end_pos),
                );
                open.insert(neighbor);
                came_from.insert(neighbor, vec![current]);
            } else if tentative_cost == *costs.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.get_mut(&neighbor).unwrap().push(current);
            }
        }
    }

    unreachable!()
}

fn get_cells_in_path(
    end: PositionWithDirection,
    came_from: &HashMap<PositionWithDirection, Vec<PositionWithDirection>>,
) -> HashSet<Position> {
    let mut total_cells = HashSet::from([end.position]);
    if came_from.contains_key(&end) {
        for cell in &came_from[&end] {
            total_cells.extend(get_cells_in_path(*cell, came_from));
        }
    }
    total_cells
}

fn get_neighboring_directions(dir: Direction) -> [Direction; 2] {
    match dir {
        Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
    }
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
    fn part1_correct_output_for_test_input1() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day16_part1(&contents);
        assert_eq!(result, 7036);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day16_part1(&contents);
        assert_eq!(result, 72400);
    }

    #[test]
    fn part2_correct_output_for_test_input1() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day16_part2(&contents);
        assert_eq!(result, 45);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day16_part2(&contents);
        assert_eq!(result, 435);
    }
}
