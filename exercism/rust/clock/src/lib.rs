use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Normalize the time to be within 0 and 23 hours and 0 and 59 minutes
        let total_minutes = Clock::calculate_total_minutes(hours, minutes);
        let minutes = Clock::normalize_minutes(total_minutes);
        let hours = Clock::normalize_hours(total_minutes);
        Clock { hours, minutes }
    }

    // Adds a number of minutes to the current time.
    // This function adds the specified number of minutes to the current time.
    // If the number of minutes to add is negative, the time subtracts the
    // specified number of minutes from the current time.
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    // Calculate total minutes from hours and minutes, normalized to be between 0 and 1439 (23 hours and 59 minutes, inclusive)
    fn calculate_total_minutes(hours: i32, minutes: i32) -> i32 {
        let mut total = (hours * 60 + minutes) % 1440;
        if total < 0 {
            total += 1440;
        }
        total
    }

    // Normalize minutes to be between 0 and 59
    fn normalize_minutes(minutes: i32) -> i32 {
        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes += 60;
        }
        minutes
    }

    // Normalize hours to be between 0 and 23
    fn normalize_hours(minutes: i32) -> i32 {
        let mut hours = minutes / 60;
        if hours < 0 {
            hours += 24;
        }
        hours
    }
}

impl fmt::Display for Clock {
    // This is the trait method that will be called
    // when you use the {} marker in a format string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // format the clock and write it to the formatter
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    // Compare two `Clock` instances for equality
    fn eq(&self, other: &Self) -> bool {
        // Check the `hours` and `minutes` fields for equality, separately
        (self.hours == other.hours) && (self.minutes == other.minutes)
    }
}
