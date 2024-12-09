use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day9_part1(&contents);
    println!("Day9 part 1 result: {result}");
}

fn day9_part1(input: &str) -> usize {
    let blocks = read_blocks_from_string(input);
    let compacted_blocks = compact_blocks(&blocks);
    compacted_blocks
        .into_iter()
        .enumerate()
        .map(|(index, id)| index * id)
        .sum()
}

fn compact_blocks(blocks: &[usize]) -> Vec<usize> {
    let mut compacted = Vec::new();
    let mut end_index = blocks.len() - 1;
    let mut start_index = 0;
    while end_index > start_index {
        while blocks[start_index] != usize::MAX {
            compacted.push(blocks[start_index]);
            start_index += 1;
        }
        while blocks[end_index] == usize::MAX {
            end_index -= 1;
        }
        compacted.push(blocks[end_index]);
        start_index += 1;
        end_index -= 1;
    }
    compacted.push(blocks[start_index]);
    compacted
}

fn read_blocks_from_string(input: &str) -> Vec<usize> {
    let mut blocks = Vec::new();
    let mut current_id = 0;
    for c in input.chars().collect::<Vec<_>>().as_slice().chunks(2) {
        let file_length = c[0].to_digit(10).unwrap();
        blocks.extend(vec![current_id; file_length as usize]);
        current_id += 1;
        if c[1] != '\n' {
            let space_length = c[1].to_digit(10).unwrap();
            blocks.extend(vec![usize::MAX; space_length as usize]);
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day9_part1(&contents);
        assert_eq!(result, 1928);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day9_part1(&contents);
        assert_eq!(result, 6398252054886);
    }
}
