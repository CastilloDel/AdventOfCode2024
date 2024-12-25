use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day23_part1(&contents);
    println!("Day23 part 1 result: {result}");
    let result = day23_part2(&contents);
    println!("Day23 part 2 result: {result}");
}

fn day23_part1(input: &str) -> usize {
    let connections = read_input(input);
    let computers = connections
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<HashSet<&str>>();
    let triplets = find_triplets(&computers, &connections);
    triplets
        .into_iter()
        .filter(|triplet| {
            triplet.0.starts_with("t") || triplet.1.starts_with("t") || triplet.2.starts_with("t")
        })
        .count()
}

// Takes like 30 min, but at this point I'm too tired to go back to it
fn day23_part2(input: &str) -> String {
    let connections = read_input(input);
    let computers = connections
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<HashSet<&str>>();
    let triplets = find_triplets(&computers, &connections);
    let mut connected_sets = triplets
        .into_iter()
        .map(|v| vec![v.0, v.1, v.2])
        .collect::<HashSet<_>>();
    while connected_sets.len() != 1 {
        let mut new_connected_sets = HashSet::new();
        for (i, set1) in connected_sets.iter().enumerate() {
            for set2 in connected_sets.iter().skip(i + 1) {
                if let Some(set) = merge_set(&connections, set1, set2) {
                    new_connected_sets.insert(set);
                }
            }
        }
        connected_sets = new_connected_sets;
    }
    let lan_party = connected_sets.into_iter().next().unwrap();
    lan_party.join(",")
}

fn merge_set<'a>(
    connections: &HashSet<(&'a str, &'a str)>,
    set1: &[&'a str],
    set2: &[&'a str],
) -> Option<Vec<&'a str>> {
    let diffs = set1
        .iter()
        .zip(set2.iter())
        .filter(|(a, b)| a != b)
        .collect::<Vec<_>>();
    if diffs.len() > 1 {
        return None;
    }
    let mut result = set1.to_vec();
    let (a, b) = diffs[0];

    if connections.contains(&(a, b)) || connections.contains(&(b, a)) {
        result.push(b);
        result.sort_unstable();
        Some(result)
    } else {
        None
    }
}

fn find_triplets<'a>(
    computers: &HashSet<&'a str>,
    connections: &HashSet<(&'a str, &'a str)>,
) -> Vec<(&'a str, &'a str, &'a str)> {
    let mut triplets = Vec::new();
    for (i, computer1) in computers.iter().enumerate() {
        for (j, computer2) in computers.iter().enumerate().skip(i + 1) {
            for computer3 in computers.iter().skip(j + 1) {
                if (connections.contains(&(*computer1, *computer2))
                    || connections.contains(&(*computer2, *computer1)))
                    && (connections.contains(&(*computer1, *computer3))
                        || connections.contains(&(*computer3, *computer1)))
                    && (connections.contains(&(*computer2, *computer3))
                        || connections.contains(&(*computer3, *computer2)))
                {
                    triplets.push((*computer1, *computer2, *computer3));
                }
            }
        }
    }
    triplets.sort_unstable();
    triplets
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

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day23_part2(&contents);
        assert_eq!(result, String::from("co,de,ka,ta"));
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day23_part2(&contents);
        assert_eq!(result, String::from(""));
    }
}
