use std::mem::swap;

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut prev = vec![0i32];
    let mut curr = Vec::<i32>::new();

    for row in triangle {
        curr.clear();
        for (i, cell) in row.into_iter().enumerate() {
            let mut sum = None;
            if prev.len() > i {
                min_or_get(&mut sum, prev[i]);
            }
            if i > 0 {
                min_or_get(&mut sum, prev[i - 1]);
            }
            curr.push(sum.unwrap() + cell);
        }

        swap(&mut prev, &mut curr);
    }

    prev.iter().min().unwrap().to_owned()
}

fn min_or_get(left: &mut Option<i32>, right: i32) {
    *left = match *left {
        None => Some(right),
        Some(l) => Some(l.min(right)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_grid;
    use crate::parameter_tests;

    struct Case {
        input: String,
        expect: i32,
    }

    fn test(c: Case) {
        let input = parse_grid::<i32>(&c.input).unwrap();
        assert_eq!(c.expect, minimum_total(input))
    }

    parameter_tests! {
        test,
        (case1: Case { input: "[[2],[3,4],[6,5,7],[4,1,8,3]]".to_string(), expect: 11 }),
        (case2: Case { input: "[[-10]]".to_string(), expect: -10 }),
    }
}
