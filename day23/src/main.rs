use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day23_part1(&contents);
    println!("Day23 part 1 result: {result}");
}

fn day23_part1(input: &str) -> usize {
    let connections = read_input(input);
    let computers = connections
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<HashSet<&str>>();
    let mut triplets = Vec::new();
    for (i, computer1) in computers.iter().enumerate() {
        for (j, computer2) in computers.iter().enumerate().skip(i) {
            for computer3 in computers.iter().skip(j) {
                if (connections.contains(&(*computer1, *computer2))
                    || connections.contains(&(*computer2, *computer1)))
                    && (connections.contains(&(*computer1, *computer3))
                        || connections.contains(&(*computer3, *computer1)))
                    && (connections.contains(&(*computer2, *computer3))
                        || connections.contains(&(*computer3, *computer2)))
                {
                    triplets.push((computer1, computer2, computer3));
                }
            }
        }
    }
    triplets
        .into_iter()
        .filter(|triplet| {
            triplet.0.starts_with("t") || triplet.1.starts_with("t") || triplet.2.starts_with("t")
        })
        .count()
}

fn read_input(input: &str) -> HashSet<(&str, &str)> {
    input
        .lines()
        .map(|line| (&line[0..2], &line[3..5]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day23_part1(&contents);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day23_part1(&contents);
        assert_eq!(result, 1227);
    }
}
