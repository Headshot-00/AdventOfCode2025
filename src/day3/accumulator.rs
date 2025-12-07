use crate::day3::UpdateError;
use log::warn;

pub struct Day3Accumulator {
    total_joltage_part1: u64,
    total_joltage_part2: u64,
}

impl Day3Accumulator {
    pub fn new() -> Self {
        Day3Accumulator {
            total_joltage_part1: 0,
            total_joltage_part2: 0,
        }
    }

    pub fn update(&mut self, input: &str) -> Result<(), UpdateError> {
        if input.is_empty() {
            warn!("Input was empty.");
            return Err(UpdateError::EmptyInput);
        }

        let digits_part_1 = largest_k_digits(input, 2);
        let joltage_part_1 = digits_part_1
            .iter()
            .fold(0u64, |acc, &d| acc * 10 + d as u64);
        self.total_joltage_part1 += joltage_part_1;

        let digits_part_2 = largest_k_digits(input, 12);
        let joltage_part_2 = digits_part_2
            .iter()
            .fold(0u64, |acc, &d| acc * 10 + d as u64);
        self.total_joltage_part2 += joltage_part_2;
        Ok(())
    }

    pub fn get_total_joltage_part1(&self) -> u64 {
        self.total_joltage_part1
    }

    pub fn get_total_joltage_part2(&self) -> u64 {
        self.total_joltage_part2
    }
}

impl Default for Day3Accumulator {
    fn default() -> Self {
        Self::new()
    }
}

fn largest_k_digits(input: &str, k: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut remaining = input.len();

    for c in input.chars() {
        let digit = c.to_digit(10).unwrap_or(0) as u8;

        while let Some(&last) = result.last() {
            if last < digit && result.len() + remaining > k {
                result.pop();
            } else {
                break;
            }
        }

        if result.len() < k {
            result.push(digit);
        }

        remaining -= 1;
    }

    result
}
