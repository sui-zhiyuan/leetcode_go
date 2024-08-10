use std::cmp;

pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut queries = queries
        .iter()
        .enumerate()
        .map(|(i, q)| {
            let left = q[0].min(q[1]);
            let right = q[0].max(q[1]);
            Query {
                left: left as usize,
                right: right as usize,
                index: i,
            }
        })
        .collect::<Vec<_>>();
    queries.sort_by_key(|q| cmp::Reverse(q.right));

    let mut monotonic_stack = MonotonicStack::new();
    let mut curr = heights.len();

    let mut result = vec![None; queries.len()];
    for Query { left, right, index } in queries {
        if heights[left] < heights[right] || left == right {
            result[index] = Some(right);
            continue;
        }

        while curr > right {
            curr -= 1;
            monotonic_stack.push(Building {
                height: heights[curr],
                index: curr,
            });
        }

        let building = monotonic_stack.find(heights[left]);
        result[index] = building.map(|f| f.index);
    }

    result
        .into_iter()
        .map(|x| x.map(|v| v as i32).unwrap_or(-1))
        .collect()
}

struct MonotonicStack {
    stack: Vec<Building>,
}

impl MonotonicStack {
    fn new() -> Self {
        MonotonicStack { stack: Vec::new() }
    }

    fn push(&mut self, building: Building) {
        if !self.stack.is_empty() {
            let target = binary_search(0, self.stack.len(), |i| {
                self.stack[i].height <= building.height
            });
            self.stack.truncate(target);
        }
        self.stack.push(building);
    }

    fn find(&mut self, height: i32) -> Option<&Building> {
        let target = binary_search(0, self.stack.len(), |i| self.stack[i].height <= height);
        if target >= 1 {
            return Some(&self.stack[target - 1]);
        }
        None
    }
}

fn binary_search(mut left: usize, mut right: usize, mut f: impl FnMut(usize) -> bool) -> usize {
    while left < right - 1 {
        let mid = left + (right - left) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    if f(left) {
        left
    } else {
        right
    }
}

struct Query {
    left: usize,
    right: usize,
    index: usize,
}

struct Building {
    height: i32,
    index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let heights = vec![6, 4, 8, 5, 2, 7];
        let queries = [[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        let result = leftmost_building_queries(heights, queries);
        assert_eq!(result, vec![2, 5, -1, 5, 2]);
    }

    #[test]
    fn test_2() {
        let heights = vec![1,2,1,2,1,2];
        let queries = [[0,0],[0,1],[0,2],[0,3],[0,4],[0,5],[1,0],[1,1],[1,2],[1,3],[1,4],[1,5],[2,0],[2,1],[2,2],[2,3],[2,4],[2,5],[3,0],[3,1],[3,2],[3,3],[3,4],[3,5],[4,0],[4,1],[4,2],[4,3],[4,4],[4,5],[5,0],[5,1],[5,2],[5,3],[5,4],[5,5]];
        let queries: Vec<Vec<i32>> = queries.iter().map(|x| x.to_vec()).collect();
        let result = leftmost_building_queries(heights, queries);
        assert_eq!(result, vec![0,1,3,3,5,5,1,1,-1,-1,-1,-1,3,-1,2,3,5,5,3,-1,3,3,-1,-1,5,-1,5,-1,4,5,5,-1,5,-1,5,5]);
    }
}
