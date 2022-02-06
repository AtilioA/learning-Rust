// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

fn max_throughput() -> u8 {
    return 221;
}

fn get_success_rate(speed: u8) -> f64 {
    if (speed >= 1 && speed <= 4) {
        return 1.00;
    } else if (speed >= 5 && speed <= 8) {
        return 0.90;
    } else if (speed == 9 || speed == 10) {
        return 0.77;
    } else {
        return 0.0;
    }
}

fn get_working_rate(speed: u8) -> f64 {
    return (speed as f64) * get_success_rate(speed);
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    return get_working_rate(speed) * (max_throughput() as f64);
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // Maybe round?
    return (production_rate_per_hour(speed) as u32) / 60;
}
