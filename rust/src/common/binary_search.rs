use std::ops::{Add, Div, Sub};

pub fn binary_search<T>(mut left: T, mut right: T, mut check: impl FnMut(T) -> bool) -> T
where
    T: PartialOrd + Copy + From<u8> + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    while left < right - 1.into() {
        let mid = (left + right) / 2.into();
        if check(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    
    if check(left) {
        left
    } else {
        right
    }
}
