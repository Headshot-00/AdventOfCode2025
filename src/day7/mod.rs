use crate::adv_errors::UpdateError;
use std::collections::VecDeque;
use std::io::BufRead;

pub fn solve<R: BufRead>(
    reader: R,
) -> Result<(u64, u64), UpdateError> {
    let mut lines: VecDeque<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .map_err(|_| UpdateError::EmptyInput)?;

    let first_line = lines.pop_front().ok_or(UpdateError::EmptyInput)?;
    let mut beam: Vec<u64> = first_line
        .bytes()
        .map(|b| match b {
            b'S' => Ok(1),
            b'.' => Ok(0),
            _ => return Err(UpdateError::InvalidInput),
        })
        .collect::<Result<_, _>>()?;
    let length = beam.len();

    let mut count: u64 = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() != length {
            return Err(UpdateError::InvalidInput);
        }
        let mut next_beam = beam.clone();
        for (i, ch) in chars.into_iter().enumerate() {
            if beam[i] > 0 && ch == '^' {
                count += 1;
                next_beam[i] = 0;
    
                if i > 0 {
                    next_beam[i - 1] += beam[i];
                }
                if i + 1 < length {
                    next_beam[i + 1] += beam[i];
                }
            }
        }
        beam = next_beam;
    }

    let worlds: u64 = beam.iter().map(|&x| x as u64).sum();

    Ok((count, worlds))
}