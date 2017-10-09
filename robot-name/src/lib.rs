extern crate rand;

use rand::Rng;

fn rand_char(from: char, to: char) -> char {
    rand::thread_rng().gen_range(from as u8, to as u8 + 1) as char
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut robot = Robot { name: String::new() };
        robot.reset_name();
        robot
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = format!(
            "{}{}{}{}{}",
            rand_char('A', 'Z'),
            rand_char('A', 'Z'),
            rand_char('0', '9'),
            rand_char('0', '9'),
            rand_char('0', '9')
        );
    }
}
