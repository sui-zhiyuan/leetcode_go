use std::cmp::Ordering;
use std::iter;

pub fn max_frequency_elements(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    let mut frequency = 0;
    let mut prev_value = None;

    let mut max_frequency = -1;
    let mut max_frequency_times = 0;

    for v in nums.into_iter().map(Some).chain(iter::once(None)) {
        if prev_value == v {
            frequency += 1;
            continue;
        }

        match frequency.cmp(&max_frequency) {
            Ordering::Greater => {
                max_frequency = frequency;
                max_frequency_times = 1;
            }
            Ordering::Equal => max_frequency_times += 1,
            Ordering::Less => (),
        }

        frequency = 1;
        prev_value = v;
    }

    max_frequency * max_frequency_times
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, max_frequency_elements(vec![1, 2, 2, 3, 1, 4]))
    }
}
