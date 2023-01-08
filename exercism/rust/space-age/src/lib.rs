const SECONDS_IN_EARTH_YEAR: f64 = 31557600.0;

const MERCURY_EARTH_YEARS: f64 = 0.2408467;
const VENUS_EARTH_YEARS: f64 = 0.61519726;
const EARTH_EARTH_YEARS: f64 = 1.0;
const MARS_EARTH_YEARS: f64 = 1.8808158;
const JUPITER_EARTH_YEARS: f64 = 11.862615;
const SATURN_EARTH_YEARS: f64 = 29.447498;
const URANUS_EARTH_YEARS: f64 = 84.016846;
const NEPTUNE_EARTH_YEARS: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / MERCURY_EARTH_YEARS
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / VENUS_EARTH_YEARS
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / EARTH_EARTH_YEARS
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / MARS_EARTH_YEARS
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / JUPITER_EARTH_YEARS
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / SATURN_EARTH_YEARS
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / URANUS_EARTH_YEARS
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / SECONDS_IN_EARTH_YEAR / NEPTUNE_EARTH_YEARS
    }
}
