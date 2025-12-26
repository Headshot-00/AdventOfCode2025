use crate::adv_errors::UpdateError;
use std::io::BufRead;

struct Acc {
    sum: u64,
    prod: u64,
}

impl Acc {
    fn new() -> Self {
        Self { sum: 0, prod: 1 }
    }

    fn update(&mut self, n: u64) {
        self.sum += n;
        self.prod *= n;
    }

    fn finalize(self, op: Op) -> Result<u64, UpdateError> {
        match op {
            Op::Add => Ok(self.sum),
            Op::Mul => Ok(self.prod),
        }
    }
}

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
}

fn read_problems<R: BufRead>(reader: R) -> Result<Vec<u64>, UpdateError> {
    let mut acc: Vec<Acc> = Vec::new();
    let mut last_line: Option<String> = None;

    for line in reader.lines() {
        let line = line.map_err(|_| UpdateError::EmptyInput)?;

        if let Some(prev) = last_line.take() {
            let fields = prev.split_whitespace();

            if acc.is_empty() {
                // The first data line initializes the accumulators
                acc = fields
                    .map(|s| {
                        let n = s.parse::<u64>().map_err(|_| {
                            UpdateError::InvalidInput(format!("\"{}\" is not a valid number!", s))
                        })?;
                        let mut a = Acc::new();
                        a.update(n);
                        Ok(a)
                    })
                    .collect::<Result<_, UpdateError>>()?;

                if acc.is_empty() {
                    return Err(UpdateError::InvalidInput("No columns found".into()));
                }
            } else {
                // All subsequent lines update existing accumulators
                let nums: Vec<u64> = fields
                    .map(|s| {
                        s.parse::<u64>().map_err(|_| {
                            UpdateError::InvalidInput(format!("\"{}\" is not a valid number!", s))
                        })
                    })
                    .collect::<Result<_, _>>()?;

                if nums.len() != acc.len() {
                    return Err(UpdateError::InvalidInput("Column count mismatch".into()));
                }

                for (a, n) in acc.iter_mut().zip(nums) {
                    a.update(n);
                }
            }
        }

        last_line = Some(line);
    }

    let ops_line = last_line.ok_or(UpdateError::EmptyInput)?;

    let ops = ops_line
        .split_whitespace()
        .map(Op::parse)
        .collect::<Result<Vec<_>, _>>()?;

    if acc.len() != ops.len() {
        return Err(UpdateError::InvalidInput(
            "Operator count does not match column count".into(),
        ));
    }

    acc.into_iter()
        .zip(ops)
        .map(|(a, op)| a.finalize(op))
        .collect()
}

pub fn solve<R: BufRead>(reader: R) -> Result<(u64, u64), UpdateError> {
    let results = read_problems(reader)?;
    let total: u64 = results.into_iter().sum();
    Ok((total, 0))
}
