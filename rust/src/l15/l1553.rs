use std::collections::{hash_map::Entry, HashMap};

pub fn min_days(n: i32) -> i32 {
    let n = n as usize;
    let mut cache = HashMap::<usize , i32>::new();
    cache.insert(0, 0);
    cache.insert(1, 1);
    cache.insert(2, 2);
    cache.insert(3, 2);


    inner(&mut cache, n)
}

fn inner(cache: &mut HashMap::<usize , i32>, n: usize) -> i32 {
    let en = cache.entry(n);
    if let Entry::Occupied(e) = en {
        return *e.get();
    }

    let mut cost = i32::MAX;
    cost = cost.min(inner(cache, n / 3) + n as i32 % 3  + 1);
    cost = cost.min(inner(cache, n / 2) + n as i32 % 2 + 1);

    *cache.entry(n).or_insert(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 10;
        let result = 4;
        assert_eq!(min_days(n), result);
    }

    #[test]
    fn test2() {
        let n = 5;
        let result = 4;
        assert_eq!(min_days(n), result);
    }

    #[test]
    fn test3() {
        let n = 1;
        let result = 1;
        assert_eq!(min_days(n), result);
    }

    #[test]
    fn test4() {
        let n = 56;
        let result = 6;
        assert_eq!(min_days(n), result);
    }
}