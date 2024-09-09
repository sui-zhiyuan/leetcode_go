use crate::common::binary_search;

pub fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
    let mut start = start;
    start.sort();

    let max = start[start.len() - 1] + d + 1;

    binary_search(0, max, |x| {
        let mut last = start[0];

        for &v in start[1..].iter() {
            if last + x - v > d {
                return true;
            }
            last = v.max(last + x);
        }

        false
    }) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let start = vec![6, 0, 3];
        let d = 2;
        let res = max_possible_score(start, d);
        assert_eq!(res, 4);
    }
}
