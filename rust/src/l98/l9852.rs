pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let queries = queries
        .iter()
        .map(|v| Query {
            start: v[0] as usize,
            end: v[1] as usize,
        })
        .collect::<Vec<_>>();
    let mut distance_change = vec![0 ; n];
    let mut result = Vec::with_capacity(queries.len());

    for (i, q) in queries.iter().enumerate() {
        let mut stack = Vec::new();
        let mix_c = distance_change[0..=q.start].iter().max().unwrap();
        if distance_change[q.end] < mix_c + q.change() {
            distance_change[q.end] = mix_c + q.change();
            stack.push(q.end);
        }

        while let Some(curr) = stack.pop() {
            for q in queries[..i].iter(){
                if q.start < curr {
                    continue;
                }

                if distance_change[q.end] < distance_change[curr] + q.change() {
                    distance_change[q.end] = distance_change[curr] + q.change();
                    stack.push(q.end);
                }
            }
        }

        let max_distance = distance_change.iter().max().unwrap();
        result.push(n as i32 - 1 - *max_distance);
    }
    result
}

struct Query {
    start: usize,
    end: usize,
}

impl Query {
    fn change(&self) -> i32 {
        self.end as i32 - self.start as i32 -1
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let n = 7;
        let queries = [[4,6],[0,3]];
        let queries = queries.iter().map(|v| v.to_vec()).collect();
        let result = shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![5,3]);
    }

    #[test]
    fn test2(){
        let n = 13;
        let queries = [[0,2],[4,6]];
        let queries = queries.iter().map(|v| v.to_vec()).collect();
        let result = shortest_distance_after_queries(n, queries);
        assert_eq!(result, vec![11,10]);
    }
}