use std::cmp::Ordering;

pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    match (x - z).abs().cmp(&(y - z).abs()) {
        Ordering::Less => 1,
        Ordering::Greater => 2,
        Ordering::Equal => 0,
    }
}
