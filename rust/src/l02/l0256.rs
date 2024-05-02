pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    // curr.0 = min cost in last round.
    // curr.1 = second min cost in last round.
    // curr.2 = index of min cost in last round.
    let curr = &mut (0i32, 0i32, 0usize);
    let next = &mut (i32::MAX, i32::MAX, 0usize);

    for col in costs.into_iter() {
        for (i, c) in col.into_iter().enumerate() {
            let v_next = if i == curr.2 { c + curr.1 } else { c + curr.0 };
            if v_next < next.0 {
                next.1 = next.0;
                next.0 = v_next;
                next.2 = i;
            } else if v_next < next.1 {
                next.1 = v_next;
            }
        }
        _ = std::mem::replace(curr, *next);
        *next = (i32::MAX, i32::MAX, 0);
    }

    curr.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn test1() {
        let costs = common::parse_grid::<i32>("[[17,2,17],[16,16,5],[14,3,19]]").unwrap();
        assert_eq!(10, min_cost(costs));
    }

    #[test]
    fn test2() {
        let costs = common::parse_grid::<i32>("[[7,6,2]]").unwrap();
        assert_eq!(2, min_cost(costs));
    }

    #[test]
    fn test3() {
        let costs = common::parse_grid::<i32>("[[3,5,3],[6,17,6],[7,13,18],[9,10,18]]").unwrap();
        assert_eq!(26, min_cost(costs));
    }
}
