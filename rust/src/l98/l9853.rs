use std::collections::BTreeMap;

pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let queries = queries
        .iter()
        .map(|v| Query {
            start: v[0] as usize,
            end: v[1] as usize,
        })
        .collect::<Vec<_>>();

    let mut collection = BTreeMap::<usize, &Query>::new();
    let mut result = Vec::with_capacity(queries.len());
    let mut change = 0;

    for q in queries.iter() {
        let mut need_remove = vec![];
        let last = collection.range(..=q.start).next_back();
        if last.is_some() && last.unwrap().1.end >= q.end {
            result.push(n - 1 - change);
            continue;
        }
        for (k, v) in collection.range(q.start..q.end) {
            if v.change() >= q.change() {
                break;
            }

            change -= v.change();
            need_remove.push(*k);
        }

        for k in need_remove {
            collection.remove(&k);
        }
        collection.insert(q.start, q);
        change += q.change();
        result.push(n - 1 - change);
    }

    result
}

struct Query {
    start: usize,
    end: usize,
}

impl Query {
    fn change(&self) -> i32 {
        self.end as i32 - self.start as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        let queries = [[2, 4], [0, 2], [0, 4]];
        let queries = queries.iter().map(|v| v.to_vec()).collect();
        let result = shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test2() {
        let n = 4;
        let queries = [[0, 3], [0, 2]];
        let queries = queries.iter().map(|v| v.to_vec()).collect();
        let result = shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![1, 1]);
    }

    #[test]
    fn test3() {
        let n = 5;
        let queries = [[1,4],[2,4]];
        let queries = queries.iter().map(|v| v.to_vec()).collect();
        let result = shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![2,2]);
    }
}
