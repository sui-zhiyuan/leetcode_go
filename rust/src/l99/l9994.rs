use std::cmp;

// cspell:ignore halfs

pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    let edges1 = edges1.iter().map(|x| Edge{a: x[0], b: x[1]}).collect::<Vec<_>>();
    let edges2 = edges2.iter().map(|x| Edge{a: x[0], b: x[1]}).collect::<Vec<_>>();

    let (_ , max_diameter1) = max_deep(&edges1, 0, -1);
    let (_ , max_diameter2) = max_deep(&edges2, 0, -1);
    let mut max_diameter1 = max_diameter1 as i32;
    let mut max_diameter2 = max_diameter2 as i32;
    if max_diameter1 < max_diameter2{
        (max_diameter1, max_diameter2) = (max_diameter2, max_diameter1);
    }

    let mut depth = [max_diameter1 /2 , (max_diameter1 - 1) /2 , max_diameter2 / 2 +1];
    depth.sort_by_key(|v| cmp::Reverse(*v));
    let diameter = [max_diameter1 /2 , (max_diameter1 - 1) /2 , max_diameter2 ];
    let diameter = diameter.into_iter().max().unwrap();
    diameter.max(&depth[0] + depth[1] +1)  -1
}

fn max_deep(edges: &[Edge], curr:i32 , parent: i32) -> (usize, usize) {
    let mut children = vec![];
    for edge in edges {
        if edge.a == curr && edge.b != parent {
            children.push(edge.b);
        } else if edge.b == curr && edge.a != parent{
            children.push(edge.a);
        }
    }

    if children.is_empty(){
        // dbg!((curr ,parent , 1, 1));
        return (1, 1);
    }

    let result = children.iter().map(|&x| max_deep(edges, x, curr)).collect::<Vec<_>>();
    if result.len() == 1{
        // dbg!((curr ,parent , result[0].0 + 1, result[0].1+1));
        return (result[0].0 + 1, result[0].1+1);
    }

    let max_diameter = result.iter().map(|x| x.1).max().unwrap();
    let mut depth = result.iter().map(|x| x.0).collect::<Vec<_>>();
    depth.sort_by_key(|v| cmp::Reverse(*v));
    let max_depth = depth[0];
    let max_diameter = cmp::max(max_diameter, depth[0] + depth[1] +1);

    // dbg!((curr ,parent , max_depth, max_diameter));
    (max_depth+1, max_diameter)
}

struct Edge {
    a: i32,
    b: i32,
}

#[cfg(test)]
mod tests{
    use crate::common;

    use super::*;

    #[test]
    fn test1(){
        let edges1 = common::parse_grid::<i32>("[[0,1],[0,2],[0,3]]").unwrap();
        let edges2 = common::parse_grid::<i32>("[[0,1]]").unwrap();
        let result = minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(result, 3);
    }
    
    #[test]
    fn test2(){
        let edges1 = common::parse_grid::<i32>("[[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]]").unwrap();
        let edges2 = common::parse_grid::<i32>("[[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]]").unwrap();
        let result = minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(result, 5);
    }

    #[test]
    fn test3(){
        let edges1: Vec<Vec<i32>> = vec![];
        let edges2 = common::parse_grid::<i32>("[[0,1],[1,2]]").unwrap();
        let result = minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test4(){
        let edges1: Vec<Vec<i32>> = vec![];
        let edges2 = common::parse_grid::<i32>("[[0,1]]").unwrap();
        let result = minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(result, 2);
    }


    #[test]
    fn test5(){
        let edges1: Vec<Vec<i32>> = common::parse_grid::<i32>("[[1,0],[2,3],[1,4],[2,1],[2,5]]").unwrap();
        let edges2 = common::parse_grid::<i32>("[[4,5],[2,6],[3,2],[4,7],[3,4],[0,3],[1,0],[1,8]]").unwrap();
        let result = minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(result, 6);
    }
}