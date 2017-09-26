use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let mut hours_offset = minutes / 60;

        if minutes < 0 {
            hours_offset -= 1;
        }

        let mut hours = (hours + hours_offset) % 24;
        let mut minutes = minutes % 60;

        if hours < 0 {
            hours += 24;
        }

        if minutes < 0 {
            minutes += 60;
        }

        Clock { hours: hours, minutes: minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
