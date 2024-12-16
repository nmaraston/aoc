use std::io::BufRead;

use super::{AocSolutionError, Solution};

const EMPTY_BLOCK: i64 = -1;

pub struct Day9Solution {}

impl Solution for Day9Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut input_buffer = String::new();
        input.read_to_string(&mut input_buffer)?;
        input_buffer.pop();

        let (mut disk_blocks, _, _) = load_disk_map(&input_buffer);

        let mut i = 0;
        while disk_blocks[i] != EMPTY_BLOCK {
            i += 1;
        }

        let mut j = disk_blocks.len() - 1;
        while disk_blocks[j] == EMPTY_BLOCK {
            j -= 1;
        }

        while i < j {
            disk_blocks[i] = disk_blocks[j];
            disk_blocks[j] = -1;

            loop {
                i += 1;
                if disk_blocks[i] == EMPTY_BLOCK {
                    break;
                }
            }

            loop {
                j -= 1;
                if disk_blocks[j] != EMPTY_BLOCK {
                    break;
                }
            }
        }

        Ok(checksum(&disk_blocks).to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut input_buffer = String::new();
        input.read_to_string(&mut input_buffer)?;
        input_buffer.pop();

        let (mut disk_blocks, file_nodes, mut free_nodes) = load_disk_map(&input_buffer);

        for file_node in file_nodes.into_iter().rev() {
            for free_node in &mut free_nodes {
                if free_node.index > file_node.index {
                    break;
                }

                // We found a free segment with sufficient space for the file blocks.
                if file_node.size <= free_node.size {
                    for i in 0..file_node.size {
                        disk_blocks[free_node.index + i] = disk_blocks[file_node.index + i];
                        disk_blocks[file_node.index + i] = -1;
                    }

                    free_node.index += file_node.size;
                    free_node.size = free_node.size - file_node.size;
                    break;
                }
            }
        }

        Ok(checksum(&disk_blocks).to_string())
    }
}

struct INode {
    index: usize,
    size: usize,
}

fn load_disk_map(input: &String) -> (Vec<i64>, Vec<INode>, Vec<INode>) {
    let mut disk_blocks = Vec::new();
    let mut file_blocks = Vec::new();
    let mut free_blocks = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let block_size = c.to_digit(10).unwrap() as usize;
        let symbol = if i % 2 == 0 {
            file_blocks.push(INode {
                index: disk_blocks.len(),
                size: block_size,
            });
            (i as i64) / 2
        } else {
            free_blocks.push(INode {
                index: disk_blocks.len(),
                size: block_size,
            });
            EMPTY_BLOCK
        };

        for _ in 0..block_size {
            disk_blocks.push(symbol)
        }
    }

    (disk_blocks, file_blocks, free_blocks)
}

fn checksum(disk_map: &Vec<i64>) -> i64 {
    disk_map
        .into_iter()
        .enumerate()
        .map(|(i, n)| (i as i64) * (*n).max(0))
        .sum()
}
