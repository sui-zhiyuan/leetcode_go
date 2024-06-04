use std::{
    collections::{HashMap, HashSet},
    io::sink,
};

pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
    let n = edges.len() + 1;
    let mut stack = vec![];
    let mut map = vec![
        vec![
            Distance {
                distance: 0,
                next: 0
            };
            n
        ];
        n
    ];
    let mut visited: HashSet<usize> = HashSet::new();
    stack.push(0);
    visited.insert(0);

    while let Some(curr) = stack.pop() {
        for v in edges.iter() {
            let &[start, end, distance] = &v[0..3] else {
                panic!()
            };
            let (mut start, mut end) = (start as usize, end as usize);
            if start != curr && end != curr {
                continue;
            }
            if start != curr {
                (start, end) = (end, start)
            }
            if visited.contains(&end) {
                continue;
            }
            for &d in visited.iter() {
                if d == start {
                    continue;
                }
                let v = map[d][start].clone();
                map[end][d] = Distance {
                    next: start,
                    distance: distance + v.distance,
                };
                map[d][end] = Distance {
                    next: v.next,
                    distance: distance + v.distance,
                };
            }
            map[start][end] = Distance {
                distance,
                next: end,
            };
            map[end][start] = Distance {
                distance,
                next: start,
            };
            stack.push(end);
            visited.insert(end);
        }
    }

    let mut result = vec![0; n];
    for i in 0..n {
        let mut children = HashMap::<usize, usize>::new();
        for j in 0..n {
            if i == j || map[i][j].distance % signal_speed != 0 {
                continue;
            }
            *children.entry(map[i][j].next).or_default() += 1;
        }
        let children = children.iter().map(|(k, v)| v).collect::<Vec<_>>();
        for (k, sk) in children.iter().enumerate() {
            for sl in children.iter().skip(k+1) {
                result[i] += (**sk * **sl) as i32;
            }
        }
    }
    result
}

#[derive(Debug, Clone)]
struct Distance {
    distance: i32,
    next: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn test1() {
        let edges =
            common::parse_grid::<i32>("[[0,1,1],[1,2,5],[2,3,13],[3,4,9],[4,5,2]]").unwrap();
        let signal_speed = 1;
        let res = vec![0, 4, 6, 6, 4, 0];
        assert_eq!(count_pairs_of_connectable_servers(edges, signal_speed), res);
    }

    #[test]
    fn test2() {
        let edges =
            common::parse_grid::<i32>("[[0,6,3],[6,5,3],[0,3,1],[3,2,7],[3,1,6],[3,4,2]]").unwrap();
        let signal_speed = 3;
        let res = vec![2,0,0,0,0,0,2];
        assert_eq!(count_pairs_of_connectable_servers(edges, signal_speed), res);
    }
}
