pub struct Day1StateMachine {
    state: i32,
    counter: i32,
}

impl Day1StateMachine {
    pub fn new() -> Self {
        Day1StateMachine {
            state: 0,
            counter: 0,
        }
    }

    pub fn update(&mut self, input: &str) {
        if input.is_empty() {
            println!("Input is empty.");
            return;
        }

        let command = &input[0..1];
        let value_str = &input[1..].trim();

        match command {
            "R" => {
                if let Ok(num) = value_str.parse::<i32>() {
                    self.state = (self.state + num).rem_euclid(99);
                } else {
                    println!("Invalid number for addition: {}", value_str);
                }
            }
            "L" => {
                if let Ok(num) = value_str.parse::<i32>() {
                    self.state = (self.state - num).rem_euclid(99);
                } else {
                    println!("Invalid number for subtraction: {}", value_str);
                }
            }
            _ => println!("Unknown command: {}", command),
        }

        if self.state == 0 {
            self.counter += 1;
        }
    }

    pub fn get_state(&self) -> i32 {
        self.state
    }

    pub fn get_zero_counter(&self) -> i32 {
        self.counter
    }
}
