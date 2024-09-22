use std::collections::{BinaryHeap, HashSet};

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }
    let routes = routes
        .into_iter()
        .map(|r| r.into_iter().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let n = routes.len();
    let mut reach = vec![vec![]; n];
    let mut source_reach = vec![];
    let mut target_reach = vec![];

    for i in 0..n {
        for j in i + 1..n {
            if routes[i].intersection(&routes[j]).next().is_some() {
                reach[i].push(j);
                reach[j].push(i);
            }
        }
        if routes[i].contains(&source) {
            source_reach.push(i);
        }
        if routes[i].contains(&target) {
            target_reach.push(i);
        }
    }

    let mut distance = vec![None; n];
    for &i in &source_reach {
        distance[i] = Some(1);
    }

    let mut queue = BinaryHeap::new();
    for &i in &source_reach {
        queue.push((1, i));
    }

    while let Some((d, i)) = queue.pop() {
        if distance[i].unwrap() < d {
            continue;
        }
        for &j in &reach[i] {
            if distance[j].is_none() || distance[j].unwrap() > d + 1 {
                distance[j] = Some(d + 1);
                queue.push((d + 1, j));
            }
        }
    }

    target_reach
        .into_iter()
        .filter_map(|i| distance[i])
        .min()
        .unwrap_or(-1)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        let source = 1;
        let target = 6;
        let result = num_buses_to_destination(routes, source, target);
        assert_eq!(result, 2);
    }
}