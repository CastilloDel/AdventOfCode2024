use std::fs;

const EMPTY: usize = usize::MAX;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day9_part1(&contents);
    println!("Day9 part 1 result: {result}");
    let result = day9_part2(&contents);
    println!("Day9 part 2 result: {result}");
}

fn day9_part1(input: &str) -> usize {
    let blocks = part1::read_blocks_from_string(input);
    let compacted_blocks = part1::compact_blocks(&blocks);
    compacted_blocks
        .into_iter()
        .enumerate()
        .map(|(index, id)| index * id)
        .sum()
}

fn day9_part2(input: &str) -> usize {
    let blocks = part2::read_blocks_from_string(input);
    let compacted_blocks = part2::compact_blocks(blocks);
    compacted_blocks
        .into_iter()
        .flat_map(|block| (0..block.length).map(move |_| block.id))
        .enumerate()
        .filter(|(_, id)| id != &EMPTY)
        .map(|(index, id)| index * id)
        .sum()
}

mod part1 {
    use super::EMPTY;

    pub fn compact_blocks(blocks: &[usize]) -> Vec<usize> {
        let mut compacted = Vec::new();
        let mut end_index = blocks.len() - 1;
        let mut start_index = 0;
        while end_index > start_index {
            while blocks[start_index] != EMPTY {
                compacted.push(blocks[start_index]);
                start_index += 1;
            }
            while blocks[end_index] == EMPTY {
                end_index -= 1;
            }
            compacted.push(blocks[end_index]);
            start_index += 1;
            end_index -= 1;
        }
        compacted.push(blocks[start_index]);
        compacted
    }

    pub fn read_blocks_from_string(input: &str) -> Vec<usize> {
        let mut blocks = Vec::new();
        let mut current_id = 0;
        for c in input.chars().collect::<Vec<_>>().as_slice().chunks(2) {
            let file_length = c[0].to_digit(10).unwrap();
            blocks.extend(vec![current_id; file_length as usize]);
            current_id += 1;
            if c[1] != '\n' {
                let space_length = c[1].to_digit(10).unwrap();
                blocks.extend(vec![EMPTY; space_length as usize]);
            }
        }
        blocks
    }
}

mod part2 {
    use super::EMPTY;

    #[derive(Debug, Clone, Copy)]
    pub struct BlockGroup {
        pub id: usize,
        pub length: usize,
    }

    pub fn compact_blocks(mut blocks: Vec<BlockGroup>) -> Vec<BlockGroup> {
        let mut block_to_move_index = blocks.len();
        while block_to_move_index > 0 {
            block_to_move_index -= 1;
            if blocks[block_to_move_index].id == EMPTY {
                continue;
            }
            if let Some(space_index) = search_swap(&mut blocks, block_to_move_index) {
                if blocks[space_index].length == blocks[block_to_move_index].length {
                    blocks.swap(space_index, block_to_move_index);
                } else {
                    let block_to_move = blocks[block_to_move_index];
                    blocks[block_to_move_index].id = EMPTY;
                    blocks[space_index].length -= block_to_move.length;
                    blocks.insert(space_index, block_to_move);
                    block_to_move_index += 1;
                }
            }
        }
        blocks
    }

    fn search_swap(blocks: &mut Vec<BlockGroup>, block_to_move_index: usize) -> Option<usize> {
        (0..block_to_move_index).find(|&possible_space_index| {
            blocks[possible_space_index].id == EMPTY
                && blocks[possible_space_index].length >= blocks[block_to_move_index].length
        })
    }

    pub fn read_blocks_from_string(input: &str) -> Vec<BlockGroup> {
        let mut blocks = Vec::new();
        let mut current_id = 0;
        for c in input.chars().collect::<Vec<_>>().as_slice().chunks(2) {
            let file_length = c[0].to_digit(10).unwrap();
            blocks.push(BlockGroup {
                id: current_id,
                length: file_length as usize,
            });
            current_id += 1;
            if c[1] != '\n' {
                let space_length = c[1].to_digit(10).unwrap();
                if space_length != 0 {
                    blocks.push(BlockGroup {
                        id: EMPTY,
                        length: space_length as usize,
                    });
                }
            }
        }
        blocks
    }
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

    #[test]
    fn part2_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day9_part2(&contents);
        assert_eq!(result, 2858);
    }

    #[test]
    fn part2_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day9_part2(&contents);
        assert_eq!(result, 6415666220005);
    }
}
