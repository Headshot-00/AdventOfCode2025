use crate::adv_errors::UpdateError;
use std::io::BufRead;

enum Op {
    Add,
    Mul,
}

impl Op {
    fn parse(s: &str) -> Result<Self, UpdateError> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(UpdateError::InvalidInput(format!(
                "\"{}\" is not a valid operator!",
                s
            ))),
        }
    }

    fn reduce(&self, values: &[u64]) -> u64 {
        match self {
            Op::Add => values.iter().copied().sum(),
            Op::Mul => values.iter().copied().product(),
        }
    }
}

fn split_symbol_space_chunks(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for c in s.chars() {
        if !c.is_whitespace() {
            if !current.is_empty() {
                result.push(current);
                current = String::new();
            }
        }
        current.push(c);
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn split_by_char_lengths(s: &str, lengths: &[usize]) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();
    let mut index = 0;

    for &len in lengths {
        let end = index + len;
        let chunk: String = chars[index..end].iter().collect();
        result.push(chunk);
        index = end;
    }

    result
}

fn read_problems<R: BufRead>(reader: R) -> Result<(Vec<u64>, Vec<u64>), UpdateError> {
    let lines: Result<Vec<String>, UpdateError> =
        reader.lines().map(|l| l.map_err(UpdateError::Io)).collect();

    let mut lines = lines?;

    for line in &mut lines {
        line.push(' '); // Hacky, but add a single whitespace to the end of each line so that parsing works the same for every row.
    }

    let last = lines.pop().ok_or(UpdateError::EmptyInput)?;

    let pattern_chunks = split_symbol_space_chunks(&last);
    let lengths: Vec<usize> = pattern_chunks.iter().map(|c| c.chars().count()).collect();

    let split_lines: Vec<Vec<String>> = lines
        .iter()
        .map(|line| split_by_char_lengths(line, &lengths))
        .collect();

    let ops: Result<Vec<Op>, UpdateError> = pattern_chunks
        .iter()
        .map(|chunk| {
            let op_str = chunk
                .chars()
                .next()
                .ok_or(UpdateError::InvalidInput("Empty chunk".into()))?
                .to_string();
            Op::parse(&op_str)
        })
        .collect();

    let ops = ops?;

    let mut td_accumulators: Vec<Vec<u64>> = pattern_chunks
        .iter()
        .map(|chunk| {
            let n = chunk.chars().count();
            if n == 0 { vec![] } else { vec![0; n - 1] }
        })
        .collect();

    let mut lr_accumulators: Vec<u64> = ops
        .iter()
        .map(|op| match op {
            Op::Add => 0,
            Op::Mul => 1,
        })
        .collect();

    for line_chunks in &split_lines {
        for (((chunk, td_slot), lr_slot), op) in line_chunks
            .iter()
            .zip(td_accumulators.iter_mut())
            .zip(lr_accumulators.iter_mut())
            .zip(ops.iter())
        {
            let n: u64 = chunk.trim().parse().map_err(|_| {
                UpdateError::InvalidInput(format!("\"{}\" could not be parsed as a number!", chunk))
            })?;
            *lr_slot = match op {
                Op::Add => *lr_slot + n,
                Op::Mul => *lr_slot * n,
            };
            for (ch, slot) in chunk
                .chars()
                .take(chunk.chars().count() - 1)
                .zip(td_slot.iter_mut())
            {
                if let Some(digit) = ch.to_digit(10) {
                    *slot = *slot * 10 + digit as u64;
                }
            }
        }
    }

    let td_result: Vec<u64> = ops
        .iter()
        .zip(td_accumulators.iter())
        .map(|(op, acc_chunk)| op.reduce(acc_chunk))
        .collect();

    Ok((lr_accumulators, td_result))
}

pub fn solve<R: BufRead>(reader: R) -> Result<(u64, u64), UpdateError> {
    let (lr, td) = read_problems(reader)?;
    let lr_total: u64 = lr.into_iter().sum();
    let td_total: u64 = td.into_iter().sum();
    Ok((lr_total, td_total))
}
