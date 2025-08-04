use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut taken_fruits = HashMap::<i32, i32>::new();

    let mut start = 0;
    let mut count = 0;
    let mut result = 0;
    for f in fruits.iter().copied() {
        if let Entry::Occupied(mut v) = taken_fruits.entry(f) {
            *v.get_mut() += 1;
            count += 1;
            result = result.max(count);
            continue;
        }

        while taken_fruits.len() >= 2 {
            let v = taken_fruits
                .entry(fruits[start])
                .and_modify(|v| *v -= 1)
                .or_default();

            if *v == 0 {
                taken_fruits.remove(&fruits[start]).unwrap();
            }

            start += 1;
            count -= 1;
        }
        let v = taken_fruits.entry(f).or_default();
        *v += 1;
        count += 1;
        result = result.max(count);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, total_fruit(vec![0, 1, 2, 2]))
    }

    #[test]
    fn test2() {
        assert_eq!(4, total_fruit(vec![1, 2, 3, 2, 2]))
    }
}
