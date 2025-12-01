pub struct Day1StateMachine {
    state: i32,
    part_1_counter: i32,
    zero_counter: i32,
    dial_size: i32,
}

impl Day1StateMachine {
    pub fn new(dial_size: i32) -> Self {
        Day1StateMachine {
            state: 50,
            part_1_counter: 0,
            zero_counter: 0,
            dial_size,
        }
    }

    pub fn update(&mut self, input: &str) {
        if input.is_empty() {
            println!("Input is empty.");
            return;
        }

        let command = &input[0..1];
        let value_str = &input[1..].trim();

        if let Ok(num) = value_str.parse::<i32>() {
            let full = num.div_euclid(self.dial_size);
            let rem = num.rem_euclid(self.dial_size);
            self.zero_counter += full;
            let delta = match command {
                "R" => rem,
                "L" => -rem,
                _ => return,
            };

            if self.state != 0 {
                self.state += delta;
                if self.state <= 0 || self.state >= self.dial_size {
                    self.zero_counter += 1;
                }
            } else {
                self.state += delta;
            }

            self.state = self.state.rem_euclid(self.dial_size);

            if self.state == 0 {
                self.part_1_counter += 1;
            }
        }
    }

    pub fn get_zero_counter(&self) -> i32 {
        self.zero_counter
    }

    pub fn get_state(&self) -> i32 {
        self.state
    }

    pub fn get_part_1_counter(&self) -> i32 {
        self.part_1_counter
    }
}
