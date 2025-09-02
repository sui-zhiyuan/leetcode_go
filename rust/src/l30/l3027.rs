use crate::common::Coordinate;
use std::cmp::Reverse;

pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points
        .into_iter()
        .map(|p| Coordinate::new(p[0], p[1]))
        .collect::<Vec<_>>();

    let mut result = 0;
    points.sort_unstable_by_key(|p| (p.x, Reverse(p.y)));
    for (alice_i, alice) in points.iter().enumerate() {
        let mut bob_i = alice_i;
        while bob_i > 0 && points[bob_i - 1].x >= points[alice_i].x {
            bob_i -= 1;
        }

        let mut min_bob_y = None;
        for bob in points[bob_i..].iter() {
            if *bob == *alice || bob.y > alice.y {
                continue;
            }

            if min_bob_y.map(|y| bob.y > y).unwrap_or(true) {
                result += 1;
                min_bob_y = Some(bob.y);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        input: Vec<[i32; 2]>,
        result: i32,
    }

    fn test(case: Case) {
        let input = case
            .input
            .into_iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(number_of_pairs(input), case.result);
    }

    parameter_tests! {
        test,
        (case0: Case{input: [[1,1],[2,2],[3,3]].to_vec() , result:0}),
        (case1: Case{input: [[6,2],[4,4],[2,6]].to_vec() , result:2}),
        (case2: Case{input: [[3,1],[1,3],[1,1]].to_vec() , result:2}),
        (case3: Case{input: [[2,5],[128653,-2370425]].to_vec() , result:1}),
        (case4: Case{input: [[2,3],[0,6],[0,1],[2,4]].to_vec() , result:3}),
    }
}
