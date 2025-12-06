use crate::day2::UpdateError;
use log::{error, warn};

pub struct Day2Accumulator {
    sum: u64,
}

impl Day2Accumulator {
    pub fn new() -> Self {
        Day2Accumulator { sum: 0 }
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

        // Get the even digit ranges in [num1, num2]
        let ranges = even_ranges(num1, num2);
        for (start, end, digits) in ranges {
            // Instead of brute forcing, we directly calculate the possible doubled up numbers
            // First we calculate 10^(number of digits / 2), having this will allow us to
            // extract the first and second half of the digit representation efficiently using
            // div and rem.
            //
            // Ex. 1234 has 4 digits so divisor becomes 10^(4/2) = 10^2 = 100
            // 1234 / 100 = 12 and 1234 % = 34
            let divisor = 10u64.pow(digits / 2);
            // Now that we have the number halves, we can directly check the range of possible
            // doubled up halves by using max and min on the lower and higher number of the
            // range respectively.
            let h_start = (start + divisor) / (divisor + 1);
            let h_end = end / (divisor + 1);
            // If the smallest part of the end digit is smaller than the largest part of the
            // beginning digit we can just skip this range.
            if h_end < h_start {
                continue;
            }
            // For all the other digits in our range of halves we can directly calculate the
            // wholes.
            for i in h_start..=h_end {
                self.sum += i + i * divisor;
            }
        }
        Ok(())
    }

    pub fn get_sum(&self) -> u64 {
        self.sum
    }
}

impl Default for Day2Accumulator {
    fn default() -> Self {
        Self::new()
    }
}

fn num_digits(mut n: u64) -> u32 {
    let mut digits = 1;
    while n >= 10 {
        n /= 10;
        digits += 1;
    }
    digits
}

fn even_ranges(num1: u64, num2: u64) -> impl Iterator<Item = (u64, u64, u32)> {
    let start_digits = num_digits(num1);
    let end_digits = num_digits(num2);

    (start_digits..=end_digits).filter_map(move |d| {
        // We only care about even digits
        if d % 2 != 0 {
            return None;
        }

        let lower = 10u64.pow(d - 1); // Lower bound for the d-digit numbers
        let upper = 10u64.pow(d) - 1; // Upper bound for the d-digit numbers

        // Adjust the bounds to fit within num1 and num2
        let start = num1.max(lower);
        let end = num2.min(upper);

        // Only add ranges where start <= end
        if start <= end {
            Some((start, end, d))
        } else {
            None
        }
    })
}
