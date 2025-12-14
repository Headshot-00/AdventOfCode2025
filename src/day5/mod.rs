use crate::adv_errors::UpdateError;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Range {
    lower: i64,
    upper: i64,
}

impl Range {
    fn new(a: i64, b: i64) -> Self {
        let lower = a.min(b);
        let upper = a.max(b);
        Range { lower, upper }
    }

    fn contains(&self, value: i64) -> bool {
        value >= self.lower && value <= self.upper
    }

    fn span(&self) -> i64 {
        self.upper - self.lower + 1
    }
}

pub fn read_ranges_ingredients<R: BufRead>(
    reader: R,
) -> Result<(Vec<Range>, Vec<i64>), UpdateError> {
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .map_err(|_| UpdateError::EmptyInput)?;

    let split_idx = lines
        .iter()
        .position(|l| l.trim().is_empty())
        .ok_or(UpdateError::InvalidInput)?;

    let (range_lines, number_lines) = lines.split_at(split_idx);

    let ranges = range_lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_once('-').ok_or(UpdateError::InvalidInput)?;

            let lower = a.parse::<i64>().map_err(|_| UpdateError::InvalidInput)?;
            let upper = b.parse::<i64>().map_err(|_| UpdateError::InvalidInput)?;

            Ok(Range::new(lower, upper))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let numbers = number_lines
        .iter()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.parse::<i64>().map_err(|_| UpdateError::InvalidInput))
        .collect::<Result<Vec<_>, _>>()?;

    Ok((ranges, numbers))
}

pub fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.lower);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for range in ranges.into_iter().skip(1) {
        if range.lower <= current.upper {
            current.upper = current.upper.max(range.upper);
        } else {
            merged.push(current);
            current = range;
        }
    }

    merged.push(current);
    merged
}

pub fn solve<R: BufRead>(reader: R) -> Result<(i64, i64), UpdateError> {
    let (ranges, ingredients) = read_ranges_ingredients(reader)?;

    let merged_ranges = merge_ranges(ranges);

    let fresh = ingredients
        .iter()
        .filter(|&&value| merged_ranges.iter().any(|r| r.contains(value)))
        .count();

    let count = merged_ranges.iter().map(|r| r.span()).sum();

    Ok((fresh as i64, count))
}
