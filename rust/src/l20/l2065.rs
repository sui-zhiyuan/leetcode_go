pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
    let mut nodes = vec![];
    for (i , &v) in values.iter().enumerate() {
        nodes.push(Node{_id: i, value: v, edges: vec![]});
    }

    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        let cost = edge[2];
        nodes[from].edges.push(Edge{to, cost});
        nodes[to].edges.push(Edge{to: from, cost});
    }
    
    let ctx = Ctx{
        nodes,
        max_time,
    };
    let mut visited = vec![false; values.len()];
    dfs(&ctx, &mut visited, 0, 0)
}

struct Ctx {
    nodes: Vec<Node>,
    max_time: i32,
}
fn dfs(ctx: &Ctx, visited: &mut Vec<bool> , curr: usize , cost: i32) -> i32 {
    let store_visit = visited[curr];
    visited[curr] = true;

    let c_node = &ctx.nodes[curr];
    // dbg!((curr, cost, &visited));
    let mut result = -1;
    if curr == 0 {
        result = ctx.nodes.iter().zip(visited.iter().copied()).filter(|(_, v)| *v).map(|(n, _)| n.value).sum();
        // dbg!(("root" , result));
    }
    for edge in c_node.edges.iter() {
        let cost = cost + edge.cost;
        if cost > ctx.max_time {
            continue;
        }
        result = dfs(ctx, visited, edge.to , cost).max(result);
    }

    visited[curr] = store_visit;

    result
}

#[derive(Debug)]
struct Node {
    _id: usize,
    value: i32,
    edges: Vec<Edge>,
}

#[derive(Debug)]
struct Edge{
    to: usize,
    cost: i32,
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let values = vec![0,32,10,43];
        let edges = common::parse_grid::<i32>("[[0,1,10],[1,2,15],[0,3,10]]").unwrap();
        let max_time = 49;
        let res = 75;
        assert_eq!(maximal_path_quality(values, edges, max_time), res);
    }

    #[test]
    fn test2() {
        let values = vec![1,2,3,4];
        let edges = common::parse_grid::<i32>("[[0,1,10],[1,2,11],[2,3,12],[1,3,13]]").unwrap();
        let max_time = 50;
        let res = 7;
        assert_eq!(maximal_path_quality(values, edges, max_time), res);
    }
}