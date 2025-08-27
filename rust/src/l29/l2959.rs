use std::iter;

use crate::common::Grid;

pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut grid = Grid::new((n, n), Option::<i32>::None);
    for road in roads {
        if grid[(road[0] as usize, road[1] as usize)].is_some() && grid[(road[0] as usize, road[1] as usize)].unwrap() <= road[2] {
            continue;
        }
        grid[(road[0] as usize, road[1] as usize)] = Some(road[2]);
        grid[(road[1] as usize, road[0] as usize)] = Some(road[2]);
    }
    for i in 0..n {
        grid[(i, i)] = Some(0);
    }
    let mut used = vec![false; n];
    count_solution(n, max_distance, &grid, 0, &mut used)
}

fn count_solution(
    n: usize,
    max_distance: i32,
    grid: &Grid<Option<i32>>,
    curr: usize,
    used: &mut [bool],
) -> i32 {
    if curr == n {
        for xy in (0..n).flat_map(|i| iter::repeat(i).zip(0..n)) {
            if !used[xy.0] || !used[xy.1] {
                continue;
            }
            match grid[xy] {
                Some(v) if v > max_distance => {
                    dbg!(&used , "too far", xy); return 0
                }
                None => {
                    dbg!(&used , "not reach", xy);return 0
                }
                _ => continue,
            }
        }
        dbg!(&used , "allow");
        return 1;
    }

    let mut result = 0;

    // remove current node
    result += count_solution(n, max_distance, grid, curr + 1 , used);

    // keep current node
    let mut state = grid.clone();
    for (i, j) in (0..n).flat_map(|i| iter::repeat(i).zip(0..n)) {
        let new_value = state[(i, curr)].and_then(|v| grid[(curr, j)].map(|w| v + w));
        if new_value.is_none() {
            continue;
        }
        let new_value = new_value.unwrap();
        if state[(i, j)].is_none() || state[(i, j)].unwrap() > new_value {
            state[(i, j)] = Some(new_value);
        }
    }

    used[curr] = true;
    result += count_solution(n, max_distance, &state, curr + 1, used);
    used[curr] = false;

    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let n = 3;
        let max_distance = 4;
        let roads = [[0,1,2],[1,2,10],[0,2,10]];
        let roads = roads.iter().map(|r| r.to_vec()).collect();
        assert_eq!(number_of_sets(n, max_distance, roads), 5);
    }

    #[test]
    fn test2() {
        let n = 3;
        let max_distance = 12;
        let roads = [[1,0,11],[1,0,16],[0,2,13]];
        let roads = roads.iter().map(|r| r.to_vec()).collect();
        assert_eq!(number_of_sets(n, max_distance, roads), 5);
    }
}