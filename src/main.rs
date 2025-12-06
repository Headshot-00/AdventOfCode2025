mod day1;
mod day2;

use day1::sm::Day1StateMachine;

use day2::accumulator::Day2Accumulator;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn day1() -> io::Result<()> {
    println!("Day 1 start");
    let input = File::open("day1input")?;
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

fn day2() -> io::Result<()> {
    println!("Day 2 start");

    let file = File::open("day2gigainput")?;
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

    println!("Total sum of doubled numbers: {}", acc.get_sum());
    Ok(())
}

fn main() -> io::Result<()> {
    //day1()?;
    day2()?;
    Ok(())
}
