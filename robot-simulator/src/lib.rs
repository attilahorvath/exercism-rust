#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (isize, isize),
    direction: Direction
}

use Direction::*;

impl Robot {
    pub fn new(x: isize, y: isize, direction: Direction) -> Self {
        Self { position: (x, y), direction: direction }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North
        };

        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South
        };

        self
    }

    pub fn advance(mut self) -> Self {
        self.position = match self.direction {
            North => (self.position.0, self.position.1 + 1),
            East => (self.position.0 + 1, self.position.1),
            South => (self.position.0, self.position.1 - 1),
            West => (self.position.0 - 1, self.position.1)
        };

        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for instruction in instructions.chars() {
            self = match instruction {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => self
            };
        }

        self
    }

    pub fn position(&self) -> (isize, isize) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
