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

#[derive(Debug)]
struct Region {
    positions: HashSet<Position>,
    area: usize,
    perimeter: usize,
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day12_part1(&contents);
    println!("Day12 part 1 result: {result}");
}

fn day12_part1(input: &str) -> usize {
    let matrix = read_input(input);
    compute_regions(&matrix)
        .into_iter()
        .map(|region| region.perimeter * region.area)
        .sum()
}

fn compute_regions(matrix: &Matrix<char>) -> Vec<Region> {
    let positions = (0..matrix.m())
        .into_iter()
        .flat_map(|i| (0..matrix.n()).into_iter().map(move |j| (i, j)))
        .collect::<Vec<Position>>();
    let mut regions = Vec::new();
    let mut already_in_region = HashSet::<Position>::new();
    for pos in positions {
        if already_in_region.contains(&pos) {
            continue;
        }
        let new_region = create_region(pos, matrix);
        already_in_region.extend(&new_region.positions);
        regions.push(new_region);
    }
    regions
}

fn create_region(pos: (usize, usize), matrix: &Matrix<char>) -> Region {
    let region_value = matrix[pos];
    let mut positions = HashSet::from([pos]);
    let mut area = 1;
    let mut perimeter = 4;
    let mut unlooked_neighbors = vec![pos];
    while !unlooked_neighbors.is_empty() {
        let pos_to_look = unlooked_neighbors.pop().unwrap();
        let neighbors = matrix
            .get_neighbors(pos_to_look)
            .into_iter()
            .filter(|&neighbor| matrix[neighbor] == region_value)
            .filter(|neighbor| !positions.contains(neighbor))
            .collect::<Vec<Position>>();
        for neighbor in neighbors {
            let adjacent_already_in_region = matrix
                .get_neighbors(neighbor)
                .into_iter()
                .filter(|adjacent| positions.contains(adjacent))
                .count();
            match adjacent_already_in_region {
                4 => perimeter -= 4,
                3 => perimeter -= 2,
                2 => perimeter += 0,
                1 => perimeter += 2,
                _ => unreachable!(),
            };
            unlooked_neighbors.push(neighbor);
            positions.insert(neighbor);
            area += 1;
        }
    }

    Region {
        positions,
        area,
        perimeter,
    }
}

fn read_input(input: &str) -> Matrix<char> {
    Matrix {
        inner: input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day12_part1(&contents);
        assert_eq!(result, 1930);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day12_part1(&contents);
        assert_eq!(result, 1371306);
    }
}
