use std::{
    cmp::Ordering,
    collections::{HashMap, LinkedList},
};

pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
    let edge_count = edges.len();
    let edges = edges
        .iter()
        .enumerate()
        .map(|(i, v)| (i, v[0] as usize, v[1] as usize, v[2]))
        .fold(
            HashMap::<usize, Vec<(usize, i32, usize)>>::new(),
            |acc, (i, s, t, v)| {
                let mut acc = acc;
                acc.entry(s).or_default().push((t, v, i));
                acc.entry(t).or_default().push((s, v, i));
                acc
            },
        );

    let mut reached = vec![Node::default(); n.try_into().unwrap()];
    for v in reached.iter_mut() {
        v.cost = i32::MAX;
    }
    reached[0].cost = 0;
    let mut queue = LinkedList::<usize>::new();
    let mut in_queue = vec![false; n.try_into().unwrap()];
    queue.push_back(0);
    in_queue[0] = true;

    // dbg!(&edges);

    while let Some(curr) = queue.pop_front() {
        in_queue[curr] = false;
        if let Some(inner) = edges.get(&curr) {
            for &(t, v, edge) in inner {
                match (reached[curr].cost + v).cmp(&reached[t].cost) {
                    Ordering::Less => {
                        reached[t].cost = reached[curr].cost + v;
                        reached[t].prev.clear();
                        reached[t].prev.push((curr, edge));
                        if !in_queue[t] {
                            queue.push_back(t);
                            in_queue[t] = true;
                        }
                    }
                    Ordering::Equal => {
                        reached[t].prev.push((curr, edge));
                    }
                    Ordering::Greater => (),
                }
            }
        }
        // dbg!(curr, &reached);
    }

    let mut ans = vec![false; edge_count];
    let mut stack = Vec::<usize>::new();
    stack.push(n as usize - 1);

    while let Some(curr) = stack.pop() {
        for &(prev, edge) in reached[curr].prev.iter() {
            if !ans[edge] {
                ans[edge] = true;
                if prev != 0 {
                    stack.push(prev);
                }
            }
        }
    }
    ans
}

#[derive(Debug, Default, Clone)]
struct Node {
    cost: i32,
    prev: Vec<(usize, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_answer() {
        assert_eq!(
            find_answer(
                6,
                vec![
                    vec![0, 1, 4],
                    vec![0, 2, 1],
                    vec![1, 3, 2],
                    vec![1, 4, 3],
                    vec![1, 5, 1],
                    vec![2, 3, 1],
                    vec![3, 5, 3],
                    vec![4, 5, 2]
                ]
            ),
            vec![true, true, true, false, true, true, true, false]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            find_answer(
                4,
                vec![vec![2, 0, 1], vec![0, 1, 1], vec![0, 3, 4], vec![3, 2, 2]]
            ),
            vec![true, false, false, true]
        );
    }
}
