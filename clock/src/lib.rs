use std::fmt;

const MINUTES_IN_A_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = (hours * 60 + minutes) % MINUTES_IN_A_DAY;

        if minutes < 0 {
            minutes += MINUTES_IN_A_DAY;
        }

        Clock(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}
