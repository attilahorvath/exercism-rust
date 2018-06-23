#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    rolls: Vec<u16>,
    remaining_pins: u16,
    final_frame: bool,
    fill_ball: bool,
}

impl Frame {
    fn new(final_frame: bool) -> Self {
        Frame {
            rolls: Vec::new(),
            remaining_pins: 10,
            final_frame,
            fill_ball: false,
        }
    }

    fn roll(&mut self, pins: u16) -> Result<bool, Error> {
        if pins > self.remaining_pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.remaining_pins -= pins;
        self.rolls.push(pins);

        if self.final_frame {
            if self.remaining_pins == 0 && self.rolls.len() < 3 {
                self.fill_ball = true;
                self.remaining_pins = 10;
            }

            Ok(self.rolls.len() > 2 || (self.rolls.len() > 1 && !self.fill_ball))
        } else {
            Ok(self.remaining_pins == 0 || self.rolls.len() > 1)
        }
    }

    fn score<'a, I>(&self, next_rolls: I) -> u16
    where
        I: IntoIterator<Item = &'a u16>,
    {
        let mut score = self.rolls.iter().sum();

        if score == 10 {
            score += next_rolls
                .into_iter()
                .take(3 - self.rolls.len())
                .sum::<u16>();
        }

        score
    }

    fn rolls(&self) -> impl Iterator<Item = &u16> {
        self.rolls.iter()
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: Option<usize>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: (0..10).map(|i| Frame::new(i == 9)).collect(),
            current_frame: Some(0),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if let Some(frame) = self.current_frame {
            if self.frames[frame].roll(pins)? {
                self.current_frame = if frame == 9 { None } else { Some(frame + 1) }
            }

            Ok(())
        } else {
            Err(Error::GameComplete)
        }
    }

    pub fn score(&self) -> Option<u16> {
        if let Some(_) = self.current_frame {
            None
        } else {
            Some(
                self.frames
                    .iter()
                    .enumerate()
                    .map(|(index, frame)| {
                        frame.score(self.frames.iter().skip(index + 1).flat_map(Frame::rolls))
                    })
                    .sum::<u16>(),
            )
        }
    }
}
