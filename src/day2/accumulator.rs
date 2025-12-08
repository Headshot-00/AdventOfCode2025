use crate::adv_errors::UpdateError;
use crate::day2::digits::{
    DigitsU64, digit_ranges, divisors_for, is_minimal_block, pow10, pow10_minus1,
};
use log::{error, warn};

pub struct Day2Accumulator {
    sum_part1: u64,
    sum_part2: u64,
}

impl Day2Accumulator {
    pub fn new() -> Self {
        Day2Accumulator {
            sum_part1: 0,
            sum_part2: 0,
        }
    }

    pub fn update(&mut self, input: &str) -> Result<(), UpdateError> {
        if input.is_empty() {
            warn!("Input was empty.");
            return Err(UpdateError::EmptyInput);
        }
        // Split on '-'
        let items = input.split_once('-');

        // Parse the substrings to u64, abort if parsing fails
        let (num1, num2) = match items {
            Some((first, second)) => {
                let first_parsed: Result<u64, _> = first.trim().parse();
                let second_parsed: Result<u64, _> = second.trim().parse();

                match (first_parsed, second_parsed) {
                    (Ok(first), Ok(second)) => (first, second), // Return parsed numbers
                    (Err(e), _) => {
                        error!("Failed to parse first item: {}", e);
                        return Err(UpdateError::InvalidInput);
                    }
                    (_, Err(e)) => {
                        error!("Failed to parse second item: {}", e);
                        return Err(UpdateError::InvalidInput);
                    }
                }
            }
            None => {
                error!("Malformed input: {}", input);
                return Err(UpdateError::InvalidInput);
            }
        };
        if num1 > num2 {
            warn!(
                "Revesed range in input: First number {} is larger than second {}!",
                num1, num2
            );
            return Err(UpdateError::ReversedRange);
        }

        // Get the digit ranges in [num1, num2]
        let ranges = digit_ranges(num1, num2);
        for (start, end, digits) in ranges {
            for &block_digits_u32 in divisors_for(digits) {
                // Instead of brute forcing, we directly calculate the possible repeated numbers.
                let block_digits = DigitsU64::new(block_digits_u32).unwrap();

                // rep_factor is the number that when multiplied by the block gives a full repeating number.
                // Example: if digits = 8 and block_digits = 2, then rep_factor = 1010101
                // Then if we have the block 12, we get block * rep_factor = 12121212
                let rep_factor = pow10_minus1(digits) / pow10_minus1(block_digits);

                // Determine the minimal and maximal blocks that, when repeated, lie within [start, end].
                // The + rep_factor -1 trick rounds up the start to the nearest multiple of rep_factor
                let block_start =
                    ((start + rep_factor - 1) / rep_factor).max(pow10(block_digits - 1));
                let block_end = (end / rep_factor).min(pow10_minus1(block_digits));

                // Skip if there is no valid block in this range
                if block_end < block_start {
                    continue;
                }
                for block in block_start..=block_end {
                    // We can easily calculate the solution to part1 at the same time
                    if digits / block_digits == DigitsU64::from(2) {
                        self.sum_part1 += block * rep_factor;
                    }
                    // Only count numbers whose repeated pattern is minimal
                    // Example: if we have block_digits 2, then the block 11 is not minimal because '1'
                    // repeats twice within the block. But the block 12 is minimal because there is no
                    // repetiton inside the block.
                    if is_minimal_block(block, block_digits) {
                        self.sum_part2 += block * rep_factor;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_sum_part1(&self) -> u64 {
        self.sum_part1
    }

    pub fn get_sum_part2(&self) -> u64 {
        self.sum_part2
    }
}

impl Default for Day2Accumulator {
    fn default() -> Self {
        Self::new()
    }
}
