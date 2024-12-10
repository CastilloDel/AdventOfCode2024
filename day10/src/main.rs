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

#[derive(Debug, Clone)]
struct Map<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Map<T> {
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
    map_ref: &'a Map<T>,
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

impl<T> Index<Position> for Map<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.inner[pos.0][pos.1]
    }
}

impl<T> IndexMut<Position> for Map<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.inner[pos.0][pos.1]
    }
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day10_part1(&contents);
    println!("Day10 part 1 result: {result}");
    let result = day10_part2(&contents);
    println!("Day10 part 2 result: {result}");
}

fn day10_part1(input: &str) -> usize {
    let map = read_input(input);
    map.iter_with_positions()
        .filter(|(_, &height)| height == 0)
        .map(|(pos, _)| pos)
        .into_iter()
        .map(|start| {
            get_reachable_positions(start, &map)
                .into_iter()
                .filter(|&pos| map[pos] == 9)
                .count()
        })
        .sum()
}

fn day10_part2(input: &str) -> u32 {
    let map = read_input(input);
    map.iter_with_positions()
        .filter(|(_, &height)| height == 0)
        .map(|(pos, _)| pos)
        .into_iter()
        .map(|start| get_number_of_paths_to_top(start, &map))
        .sum()
}

fn get_reachable_positions(start: Position, map: &Map<u32>) -> HashSet<Position> {
    let mut reachable = HashSet::new();
    let mut starters = HashSet::from([start]);
    for _height in 1..=9 {
        starters = starters
            .into_iter()
            .flat_map(|start| get_reachable_in_one_step(start, map))
            .collect();
        reachable.extend(&starters);
    }
    reachable
}

fn get_number_of_paths_to_top(start: Position, map: &Map<u32>) -> u32 {
    let mut starters = vec![start];
    for _height in 1..=9 {
        starters = starters
            .into_iter()
            .flat_map(|start| get_reachable_in_one_step(start, map))
            .collect();
    }
    starters.len() as u32
}

fn get_reachable_in_one_step(pos: Position, map: &Map<u32>) -> Vec<Position> {
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let starting_height = map[pos];
    directions
        .into_iter()
        .filter_map(move |dir| map.get_neighbor_in_direction(pos, dir))
        .filter(|&pos| map[pos] == starting_height + 1)
        .collect()
}

fn read_input(input: &str) -> Map<u32> {
    Map {
        inner: input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<_>>>(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day10_part1(&contents);
        assert_eq!(result, 36);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day10_part1(&contents);
        assert_eq!(result, 574);
    }

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day10_part2(&contents);
        assert_eq!(result, 81);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day10_part2(&contents);
        assert_eq!(result, 1238);
    }
}
