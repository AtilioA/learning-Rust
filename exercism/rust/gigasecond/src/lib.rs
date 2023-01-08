use time::{Duration, PrimitiveDateTime as DateTime};

const GIGASECOND: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let datetime = start.checked_add(Duration::seconds(GIGASECOND));
    match datetime {
        Some(datetime) => datetime,
        None => panic!("Gigasecond::after() failed"),
    }
}
