use std::{
    collections::HashSet,
    fs,
    ops::{Index, IndexMut},
};

type Position = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
    Guard,
}

#[derive(Debug, Clone)]
struct Map {
    inner: Vec<Vec<Cell>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Map {
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
}

impl Index<Position> for Map {
    type Output = Cell;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.inner[pos.0][pos.1]
    }
}

impl IndexMut<Position> for Map {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.inner[pos.0][pos.1]
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day6_part1(&contents);
    println!("Day6 part 1 result: {result}");
    let result = day6_part2(&contents);
    println!("Day6 part 2 result: {result}");
}

fn day6_part1(input: &str) -> usize {
    let (guard, map) = read_input(input);
    let mut cells = get_cells_visited_before_leaving(guard, &map);
    // Add the original pos if the guard didn't pass through it again
    cells.insert(guard.position);
    cells.len()
}

fn day6_part2(input: &str) -> usize {
    let (guard, map) = read_input(input);
    get_cells_visited_before_leaving(guard, &map)
        .into_iter()
        .filter(|&cell| {
            let mut modified_map = map.clone();
            modified_map[cell] = Cell::Obstacle;
            check_loop(guard, modified_map)
        })
        .count()
}

fn check_loop(mut guard: Guard, map: Map) -> bool {
    let mut visited = HashSet::new();
    loop {
        let next_pos = map.get_neighbor_in_direction(guard.position, guard.direction);
        if let None = next_pos {
            return false;
        }
        let next_pos = next_pos.unwrap();

        if map[next_pos] == Cell::Obstacle {
            guard.direction = get_next_direction_clockwise(guard.direction);
        } else {
            guard.position = next_pos;
        }
        if visited.contains(&guard) {
            return true;
        }
        visited.insert(guard);
    }
}

fn get_cells_visited_before_leaving(mut guard: Guard, map: &Map) -> HashSet<Position> {
    let mut visited = HashSet::new();
    loop {
        let next_pos = map.get_neighbor_in_direction(guard.position, guard.direction);
        if let None = next_pos {
            break;
        }
        let next_pos = next_pos.unwrap();

        if map[next_pos] == Cell::Obstacle {
            guard.direction = get_next_direction_clockwise(guard.direction);
        } else {
            guard.position = next_pos;
            visited.insert(next_pos);
        }
    }
    visited
}

fn get_next_direction_clockwise(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
    }
}

fn read_input(input: &str) -> (Guard, Map) {
    let mut inner = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Cell::Guard,
                    '#' => Cell::Obstacle,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect::<Vec<Vec<Cell>>>();

    let guard_position = (0..inner.len())
        .flat_map(|i| (0..inner[0].len()).map(move |j| (i, j)))
        .find(|pos| inner[pos.0][pos.1] == Cell::Guard)
        .unwrap();
    inner[guard_position.0][guard_position.1] = Cell::Empty;
    (
        Guard {
            direction: Direction::Up,
            position: guard_position,
        },
        Map { inner },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day6_part1(&contents);
        assert_eq!(result, 41);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day6_part1(&contents);
        assert_eq!(result, 4656);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day6_part2(&contents);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day6_part2(&contents);
        assert_eq!(result, 1575);
    }
}
