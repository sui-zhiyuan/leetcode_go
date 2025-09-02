use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let classes = classes
        .into_iter()
        .map(|v| Class {
            pass: v[0],
            all: v[1],
        })
        .collect::<Vec<_>>();

    let mut extra_gain = BinaryHeap::with_capacity(extra_students as usize + 1);
    for extra in 1..=extra_students {
        let mut found_round = false;
        for c in classes.iter() {
            let prev = extra - 1;
            let eg = (c.pass + extra) as f64 / (c.all + extra) as f64
                - (c.pass + prev) as f64 / (c.all + prev) as f64;

            if extra_gain.len() < (extra_students as usize) {
                extra_gain.push(ReverseOrderedFloat(eg));
                found_round = true;
            } else {
                let mut top = extra_gain.peek_mut().unwrap();
                if *top > ReverseOrderedFloat(eg) {
                    *top = ReverseOrderedFloat(eg);
                    found_round = true;
                }
            }
        }
        if !found_round {
            break;
        }
    }

    let all: f64 = classes
        .iter()
        .map(|v| v.pass as f64 / v.all as f64)
        .chain(extra_gain.into_iter().map(|e| e.0))
        .sum();

    all / (classes.len() as f64)
}

struct Class {
    pass: i32,
    all: i32,
}

#[derive(Debug, PartialEq)]
struct ReverseOrderedFloat(f64);

impl Eq for ReverseOrderedFloat {}

impl PartialOrd for ReverseOrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ReverseOrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap().reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let classes = [[1, 2], [3, 5], [2, 2]];
        let classes = classes.into_iter().map(|c| c.to_vec()).collect::<Vec<_>>();
        assert_eq!(0.7833333333333333, max_average_ratio(classes, 2))
    }

    #[test]
    fn t1() {
        let result = ReverseOrderedFloat("10.0".parse().unwrap()).cmp(&ReverseOrderedFloat(20.0));
        assert_eq!(Ordering::Greater, result,);
    }
}
