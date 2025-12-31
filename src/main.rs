mod adv_errors;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use adv_errors::UpdateError;

use day1::sm::Day1StateMachine;

use day2::accumulator::Day2Accumulator;

use day3::accumulator::Day3Accumulator;

use day4::solver::Day4Solver;

use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "advent_of_code_2025")]
#[command(about = "Rusty solutions :3")]
struct Args {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(short, long, default_value = "day%ninput")]
    input: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Mode {
    All,
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    //Day10,
    //Day11,
    //Day12,
}

// tiny helper to wrap timing around any day function
fn time_day(
    name: &str,
    f: fn(&PathBuf) -> Result<(), UpdateError>,
    input: &PathBuf,
) -> Result<(), UpdateError> {
    println!("=== {} start ===", name);
    let start = Instant::now();
    let result = f(input);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3?}", elapsed);
    println!("===  {} end  ===\n", name);
    result
}

fn day_input_path(pattern: &PathBuf, day_name: &str) -> PathBuf {
    let s = pattern.to_string_lossy();

    if s.contains("%n") {
        let day_number = &day_name[3..];
        let day_number = format!("{:0>2}", day_number.parse::<u32>().unwrap());
        PathBuf::from(s.replace("%n", &day_number))
    } else {
        pattern.clone()
    }
}

fn day1(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let mut machine = Day1StateMachine::new(100);

    for line in reader.lines() {
        let line = line?;
        machine.update(&line);
    }
    println!("Zero Counter: {}", machine.get_part_1_counter());
    println!("Wrap Counter: {}", machine.get_zero_counter());
    println!("Final State is: {}", machine.get_state());
    Ok(())
}

fn day2(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut acc = Day2Accumulator::new();

    let mut buffer = [0u8; 8192]; // 8KB buffer
    let mut leftover = String::new(); // stores partial ranges across buffer boundaries

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break; // EOF
        }

        let chunk = &buffer[..n];
        let s = std::str::from_utf8(chunk).expect("File must be valid UTF-8");

        let mut start = 0;
        for (i, c) in s.char_indices() {
            if c == ',' {
                let piece = &s[start..i];
                let range_str = format!("{}{}", leftover, piece);
                leftover.clear();

                let trimmed = range_str.trim();
                if !trimmed.is_empty() {
                    if let Err(e) = acc.update(trimmed) {
                        eprintln!("Failed to process '{}': {}", trimmed, e);
                    }
                }

                start = i + c.len_utf8();
            }
        }

        // Save leftover for next buffer read
        leftover.push_str(&s[start..]);
    }

    // process any remaining piece after the last comma
    let trimmed = leftover.trim();
    if !trimmed.is_empty() {
        if let Err(e) = acc.update(trimmed) {
            eprintln!("Failed to process '{}': {}", trimmed, e);
        }
    }

    println!("Total sum of doubled numbers: {}", acc.get_sum_part1());
    println!("Total sum of all repeated numbers: {}", acc.get_sum_part2());
    Ok(())
}

fn day3(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let mut acc = Day3Accumulator::new();

    for line in reader.lines() {
        let line = line?;
        if let Err(e) = acc.update(&line) {
            eprintln!("Failed to process '{}': {}", line, e);
        };
    }
    println!(
        "Maximum joltage for part 1 is: {}",
        acc.get_total_joltage_part1()
    );
    println!(
        "Maximum joltage for part 2 is: {}",
        acc.get_total_joltage_part2()
    );
    Ok(())
}

fn day4(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let mut solver = Day4Solver::default();

    for line in reader.lines() {
        let line = line?;
        if let Err(e) = solver.add_row(&line) {
            eprintln!("Failed to process '{}': {}", line, e);
        };
    }

    solver.finalize_input();

    let mut firstloop = true;
    let mut total_movable = 0;
    loop {
        let accessible = solver.solve().map_err(|e| {
            eprintln!("Solver failed: {:?}", e);
            io::Error::new(io::ErrorKind::Other, "Solver failed")
        })?;
        if firstloop {
            println!(
                "The number of accessible crates after the first step is: {}",
                accessible
            );
            firstloop = false;
        }
        if accessible == 0 {
            break;
        }
        total_movable += accessible;
    }
    println!("The total number of movable crates is: {}", total_movable);

    Ok(())
}

fn day5(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let (num_fresh, total_ranges) = day5::solve(reader).unwrap();

    println!("The number of fresh ingredients is: {}", num_fresh);
    println!(
        "The total number of ingredients that could be considered fresh is: {}",
        total_ranges
    );
    Ok(())
}

fn day6(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let (problem_sum, _) = day6::solve(reader).unwrap();

    println!(
        "The sum of the solutions to all the problems is: {}",
        problem_sum
    );

    Ok(())
}

fn day7(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let (splits, worlds) = day7::solve(reader).unwrap();

    println!("The number of splits is: {}", splits);

    println!("The number of parallel worlds is {}", worlds);

    Ok(())
}

fn day8(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let (top3_product, final_x_product) = day8::solve(reader, 1000).unwrap();

    println!(
        "The product of the three biggest graphs after connecting 1000 edges is: {}",
        top3_product
    );

    println!(
        "The product of the x coordinates of the two points comprising the final edge is: {}",
        final_x_product
    );

    Ok(())
}

fn day9(input: &std::path::PathBuf) -> Result<(), UpdateError> {
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let (biggest, biggest_inside) = day9::solve(reader).unwrap();

    println!("The biggest possible rectangle has area {} units", biggest);

    println!(
        "The biggest possible rectangle containing only green or red tiles has area {} units",
        biggest_inside
    );

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let day_map: &[(&str, Mode, fn(&PathBuf) -> Result<(), UpdateError>)] = &[
        ("Day1", Mode::Day1, day1),
        ("Day2", Mode::Day2, day2),
        ("Day3", Mode::Day3, day3),
        ("Day4", Mode::Day4, day4),
        ("Day5", Mode::Day5, day5),
        ("Day6", Mode::Day6, day6),
        ("Day7", Mode::Day7, day7),
        ("Day8", Mode::Day8, day8),
        ("Day9", Mode::Day9, day9),
    ];

    match args.mode {
        Mode::All => {
            for (name, _, func) in day_map {
                let day_input = day_input_path(&args.input, name);
                if !day_input.exists() {
                    eprintln!("Input file {:?} does not exist, skipping", day_input);
                    continue;
                }
                if let Err(e) = time_day(name, *func, &day_input) {
                    match e {
                        UpdateError::EmptyInput => eprintln!("{} failed: input was empty!", name),
                        UpdateError::InvalidInput(msg) => {
                            eprintln!("{} failed: invalid input: {}", name, msg)
                        }
                        UpdateError::Io(io_err) => {
                            eprintln!("{} failed: IO error: {}", name, io_err)
                        }
                    }
                }
            }
        }
        _ => {
            if let Some((name, _, func)) = day_map.iter().find(|(_, m, _)| *m == args.mode) {
                let day_input = day_input_path(&args.input, name);
                if let Err(e) = time_day(name, *func, &day_input) {
                    match e {
                        UpdateError::EmptyInput => eprintln!("{} failed: input was empty!", name),
                        UpdateError::InvalidInput(msg) => {
                            eprintln!("{} failed: invalid input: {}", name, msg)
                        }
                        UpdateError::Io(io_err) => {
                            eprintln!("{} failed: IO error: {}", name, io_err)
                        }
                    }
                }
            } else {
                eprintln!("Unknown mode {:?}", args.mode);
            }
        }
    }

    Ok(())
}
