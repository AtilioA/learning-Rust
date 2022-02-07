// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    return (dividend / divisor, dividend % divisor);
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    return iter
        // Enumerate the items in the iterator
        .enumerate()
        // Keep only the even ones
        .filter(|(i, x)| i % 2 == 0)
        // Use closure to create iterator without the indexes (we only want the items)
        .map(|(i, x)| x);

    // Alternative version:
    //     return iter.enumerate().filter_map(|(i, x)| {
    //         if i % 2 == 0 {
    //             return Some(x);
    //         } else {
    //             return None;
    //         }
    //     });

    // Naive:
    // iter.step_by(2);
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let origin = Position(0, 0);
        return (self.0 - origin.0).abs() + (self.1 - origin.1).abs();
    }
}
