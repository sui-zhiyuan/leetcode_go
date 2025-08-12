use std::collections::HashSet;

pub fn reordered_power_of2(n: i32) -> bool {
    let mut value = 1;
    let mut available = HashSet::new();
    loop {
        available.insert(*count_digits(value));
        value = match value.checked_mul(2) {
            Some(v) => v,
            None => break,
        }
    }

    available.contains(&*count_digits(n))
}

fn count_digits(v: i32) -> Box<[i32; 10]> {
    let mut v = v;
    let mut result = Box::<[i32; 10]>::default();
    while v > 0 {
        let digit = v % 10;
        v /= 10;
        result[digit as usize] += 1;
    }
    result
}
