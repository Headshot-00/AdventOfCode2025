mod day1;

use day1::sm::Day1StateMachine;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn day1() -> io::Result<()> {
    println!("Day 1 start");
    let file = File::open("day1input")?;
    let reader = BufReader::new(file);

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

fn main() -> io::Result<()> {
    day1()?;
    Ok(())
}
