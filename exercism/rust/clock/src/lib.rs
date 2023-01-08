use std::fmt;

#[derive(Debug)]
pub struct Clock;


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", 12, 34)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        unimplemented!(
            "Construct a new Clock from {} hours and {} minutes",
            hours,
            minutes
        );
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
