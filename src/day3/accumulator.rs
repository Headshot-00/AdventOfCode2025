use crate::day3::UpdateError;
use log::warn;

pub struct Day3Accumulator {
    total_joltage: u64,
}

impl Day3Accumulator {
    pub fn new() -> Self {
        Day3Accumulator { total_joltage: 0 }
    }

    pub fn update(&mut self, input: &str) -> Result<(), UpdateError> {
        if input.is_empty() {
            warn!("Input was empty.");
            return Err(UpdateError::EmptyInput);
        }
        let mut first_joltage = 0;
        let mut second_joltage = 0;
        for char in input.chars() {
            let joltage = char.to_digit(10).unwrap_or(0);
            if joltage > first_joltage {
                first_joltage = joltage;
                second_joltage = 0;
            } else if joltage > second_joltage {
                second_joltage = joltage;
            }
        }
        if second_joltage == 0 {
            second_joltage = first_joltage;
            first_joltage = 0;
            for char in input[0..input.len() - 1].chars() {
                let joltage = char.to_digit(10).unwrap_or(0);
                if joltage > first_joltage {
                    first_joltage = joltage;
                }
            }
        }
        self.total_joltage += second_joltage as u64 + 10 * first_joltage as u64;
        Ok(())
    }

    pub fn get_total_joltage(&self) -> u64 {
        self.total_joltage
    }
}

impl Default for Day3Accumulator {
    fn default() -> Self {
        Self::new()
    }
}
