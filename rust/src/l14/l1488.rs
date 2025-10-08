use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut last_rain = HashMap::<i32, usize>::new();

    let mut drain = vec![-1; rains.len()];
    // 等效 并查集
    let mut next_available_drain = vec![0; rains.len()];

    for (day, &lake) in rains.iter().enumerate() {
        if lake == 0 {
            next_available_drain[day] = day;
            continue;
        }
        next_available_drain[day] = day + 1;
        let mut occupied = match last_rain.entry(lake) {
            Entry::Vacant(v) => {
                v.insert(day);
                continue;
            }
            Entry::Occupied(o) => o,
        };

        let mut available_day = *occupied.get();
        let mut checked_days = Vec::new();
        let mut found = false;
        checked_days.push(available_day);
        // 防止预先构造执行下一天指针超限制
        while available_day <= day {
            let next_day = next_available_drain[available_day];
            checked_days.push(next_day);
            if next_day == available_day {
                available_day = next_day;
                found = true;
                break;
            }
            available_day = next_day;
        }

        if !found {
            return vec![];
        }

        drain[available_day] = lake;
        for c in checked_days {
            next_available_drain[c] = available_day + 1;
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
