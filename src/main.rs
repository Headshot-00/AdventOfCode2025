mod adv_errors;
mod day1;
mod day2;
mod day3;
mod day4;
mod day9;

use day1::sm::Day1StateMachine;

use day2::accumulator::Day2Accumulator;

use day3::accumulator::Day3Accumulator;

use day4::solver::Day4Solver;

use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "advent_of_code_2025")]
#[command(about = "Rusty solutions :3")]
struct Args {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(short, long, default_value = "day2input")]
    input: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Mode {
    Day1,
    Day2,
    Day3,
    Day4,
    Day9,
}

fn day1(input: &std::path::Path) -> io::Result<()> {
    println!("Day 1 start");
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

fn day2(input: &std::path::Path) -> io::Result<()> {
    println!("Day 2 start");

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

fn day3(input: &std::path::Path) -> io::Result<()> {
    println!("Day 3 start");
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

fn day4(input: &std::path::Path) -> io::Result<()> {
    println!("Day 4 start");
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
    solver.init_gpu().unwrap();

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

fn day9(input: &std::path::Path) -> io::Result<()> {
    println!("Day 9 start");
    let input = File::open(input)?;
    let reader = BufReader::new(input);

    let points = day9::read_points(reader).unwrap();

    let biggest = day9::find_biggest_rectangle_simple(&points);

    println!("The biggest possible rectangle has area {} units", biggest);

    let biggest = day9::find_biggest_rectangle_polygon(points);

    println!(
        "The biggest possible rectangle containing only green or red tiles has area {} units",
        biggest
    );

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.mode {
        Mode::Day1 => day1(&args.input)?,
        Mode::Day2 => day2(&args.input)?,
        Mode::Day3 => day3(&args.input)?,
        Mode::Day4 => day4(&args.input)?,
        Mode::Day9 => day9(&args.input)?,
    }
    Ok(())
}
