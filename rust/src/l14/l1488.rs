use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut last_rain = HashMap::<i32, usize>::new();

    let mut drain = vec![-1; rains.len()];

    for (day, &lake) in rains.iter().enumerate() {
        if lake == 0 {
            continue;
        }
        let mut occupied = match last_rain.entry(lake) {
            Entry::Vacant(v) => {
                v.insert(day);
                continue;
            }
            Entry::Occupied(o) => o,
        };

        let mut found = false;
        for drain_day in *occupied.get()..day {
            if rains[drain_day] != 0 || drain[drain_day] != -1 {
                continue;
            }
            drain[drain_day] = lake;
            found = true;
            break;
        }

        if !found {
            return vec![];
        }
        occupied.insert(day);
    }

    for (rain, drain) in rains.iter().zip(drain.iter_mut()) {
        if *drain == -1 && *rain == 0 {
            *drain = 1;
        }
    }

    drain
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        rains: Vec<i32>,
        drain: Vec<i32>,
    }

    fn test(c: Case) {
        assert_eq!(c.drain, avoid_flood(c.rains))
    }

    parameter_tests! {
        test,
        (case1: Case {
            rains: vec![1, 2, 3, 4],
            drain: vec![-1, -1, -1, -1],
        }),
        (case2: Case {
            rains: vec![1,2,0,0,2,1],
            drain: vec![-1,-1,2,1,-1,-1],
        }),
        (case3: Case {
            rains: vec![1,2,0,1,2],
            drain: vec![],
        }),
    }
}
