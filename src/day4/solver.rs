use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use log::warn;
use crate::adv_errors::UpdateError;

pub struct Day4Solver {
    matrix: Vec<Vec<i8>>,       // only used for input parsing
    kernel: Vec<f32>,           // 3x3 kernel
    activation: f32,
    width: usize,
    height: usize,
    buffer_a: Vec<f32>,         // ping-pong buffer
    buffer_b: Vec<f32>,
    iteration: u8,
}

impl Day4Solver {
    pub fn new(kernel: Vec<f32>, activation: f32) -> Self {
        Day4Solver {
            matrix: Vec::new(),
            kernel,
            activation,
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
                _ => Err(UpdateError::InvalidInput),
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
        self.buffer_a = self.matrix
            .iter()
            .flat_map(|r| r.iter().map(|&v| v as f32))
            .collect();

        // initialize ping-pong buffer_b
        self.buffer_b = vec![0.0; len];

        self.iteration = 0;
    }

    pub fn solve(&mut self) -> Result<u32, UpdateError> {
        let (input_buf, output_buf) = if self.iteration % 2 == 0 {
            (&self.buffer_a, &mut self.buffer_b)
        } else {
            (&self.buffer_b, &mut self.buffer_a)
        };

        self.iteration += 1;

        let accessible = AtomicU32::new(0);
        let width = self.width;
        let height = self.height;
        let k = &self.kernel;
        let activation = self.activation;

        output_buf
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let idx = y * width + x;

                    if input_buf[idx] <= 0.0 {
                        row[x] = 0.0;
                        continue;
                    }

                    let mut sum = 0.0;

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

                            let n = input_buf[sy as usize * width + sx as usize];
                            if n > 0.0 {
                                let ki = (dy + 1) * 3 + (dx + 1);
                                sum += n * k[ki as usize];
                            }
                        }
                    }

                    if sum >= activation {
                        row[x] = 1.0;
                    } else {
                        row[x] = -1.0;
                        accessible.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });

        Ok(accessible.load(Ordering::Relaxed))
    }
}

impl Default for Day4Solver {
    fn default() -> Self {
        Self::new(
            vec![
                1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0,
                1.0 / 8.0, 0.0,       1.0 / 8.0,
                1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0,
            ],
            0.5,
        )
    }
}