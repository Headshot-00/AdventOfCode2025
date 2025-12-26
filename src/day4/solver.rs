use crate::adv_errors::UpdateError;
use log::warn;
use rayon::prelude::*;

pub struct Day4Solver {
    matrix: Vec<Vec<i8>>, // only used for input parsing
    kernel: Vec<u8>,      // 3x3 kernel
    threshold: u8,
    width: usize,
    height: usize,
    buffer_a: Vec<i8>, // ping-pong buffer
    buffer_b: Vec<i8>,
    iteration: i8,
}

impl Day4Solver {
    pub fn new(kernel: Vec<u8>, threshold: u8) -> Self {
        Day4Solver {
            matrix: Vec::new(),
            kernel,
            threshold,
            width: 0,
            height: 0,
            buffer_a: Vec::new(),
            buffer_b: Vec::new(),
            iteration: 0,
        }
    }

    pub fn add_row(&mut self, line: &str) -> Result<(), UpdateError> {
        if line.is_empty() {
            warn!("Ignored empty line.");
            return Ok(());
        }

        let row: Vec<i8> = line
            .chars()
            .map(|c| match c {
                '@' => Ok(1),
                '.' => Ok(0),
                _ => Err(UpdateError::InvalidInput(format!(
                    "Invalid grid character: {}",
                    c
                ))),
            })
            .collect::<Result<_, _>>()?;

        self.matrix.push(row);
        Ok(())
    }

    pub fn finalize_input(&mut self) {
        self.height = self.matrix.len();
        self.width = self.matrix.first().map(|r| r.len()).unwrap_or(0);

        let len = self.width * self.height;

        // flatten input into buffer_a
        self.buffer_a = self
            .matrix
            .iter()
            .flat_map(|r| r.iter().map(|&v| v as i8))
            .collect();

        // initialize ping-pong buffer_b
        self.buffer_b = vec![0; len];

        self.iteration = 0;
    }

    pub fn solve(&mut self) -> Result<u32, UpdateError> {
        let (input_buf, output_buf) = if self.iteration % 2 == 0 {
            (&self.buffer_a, &mut self.buffer_b)
        } else {
            (&self.buffer_b, &mut self.buffer_a)
        };

        self.iteration += 1;

        let width = self.width;
        let height = self.height;
        let k = &self.kernel;

        // Compute accessible counts per row in parallel
        let row_accessibles: Vec<u32> = output_buf
            .par_chunks_mut(width)
            .enumerate()
            .map(|(y, row)| {
                let mut accessible_row: u32 = 0;

                for x in 0..width {
                    let idx = y * width + x;

                    if input_buf[idx] <= 0 {
                        row[x] = 0;
                        continue;
                    }

                    let mut active_neighbors: u8 = 0;

                    for dy in -1..=1 {
                        let sy = y as isize + dy;
                        if sy < 0 || sy >= height as isize {
                            continue;
                        }

                        for dx in -1..=1 {
                            let sx = x as isize + dx;
                            if sx < 0 || sx >= width as isize {
                                continue;
                            }

                            if input_buf[sy as usize * width + sx as usize] > 0 {
                                let ki = (dy + 1) * 3 + (dx + 1);
                                active_neighbors += k[ki as usize];
                            }
                        }
                    }

                    if active_neighbors >= self.threshold {
                        row[x] = 1;
                    } else {
                        row[x] = -1;
                        accessible_row += 1;
                    }
                }

                accessible_row
            })
            .collect();

        // Sum the accessible counts from all rows
        let accessible_total: u32 = row_accessibles.iter().sum();

        Ok(accessible_total)
    }
}

impl Default for Day4Solver {
    fn default() -> Self {
        Self::new(vec![1, 1, 1, 1, 0, 1, 1, 1, 1], 4)
    }
}
