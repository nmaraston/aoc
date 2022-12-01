use std::io::BufRead;

use super::Solution;

pub struct Day1Solution {}

impl Solution for Day1Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut result: u32 = 0;
        let mut running_sum: u32 = 0;
        for line in input.lines() {
            let line = line?;
            if line.len() == 0 {
                if running_sum > result {
                    result = running_sum;
                }
                running_sum = 0;
            } else {
                running_sum += line.parse::<u32>().unwrap();
            }
        }

        if running_sum > result {
            result = running_sum;
        }

        Ok(result.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut heap = ThreeHeap::new();
        let mut running_sum: u32 = 0;
        for line in input.lines() {
            let line = line?;
            if line.len() == 0 {
                heap.submit(running_sum);
                running_sum = 0;
            } else {
                running_sum += line.parse::<u32>().unwrap();
            }
        }

        heap.submit(running_sum);

        Ok(heap.sum().to_string())
    }
}

struct ThreeHeap {
    max_sums: [u32; 3],
}

impl ThreeHeap {
    fn new() -> Self {
        Self {
            max_sums: [0, 0, 0],
        }
    }

    fn submit(&mut self, num: u32) {
        if num > self.max_sums[0] {
            self.max_sums[2] = self.max_sums[1];
            self.max_sums[1] = self.max_sums[0];
            self.max_sums[0] = num;
        } else if num > self.max_sums[1] {
            self.max_sums[2] = self.max_sums[1];
            self.max_sums[1] = num;
        } else if num > self.max_sums[2] {
            self.max_sums[2] = num;
        }
    }

    fn sum(&self) -> u32 {
        self.max_sums[0] + self.max_sums[1] + self.max_sums[2]
    }
}
