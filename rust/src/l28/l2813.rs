use std::{cmp, collections::HashSet};

pub fn find_maximum_elegance(items: Vec<Vec<i32>>, k: i32) -> i64 {
    let k = k as usize;
    let mut items = items
        .iter()
        .map(|item| Item {
            profit: item[0],
            elegance: item[1],
        })
        .collect::<Vec<_>>();
    items.sort_by_key(|v| cmp::Reverse(v.profit));

    let mut find = HashSet::<i32>::new();
    let (top, other): (Vec<_>, Vec<_>) = items.into_iter().partition(|v| {
        if find.contains(&v.elegance) {
            false
        } else {
            find.insert(v.elegance);
            true
        }
    });

    let top = top
        .iter()
        .enumerate()
        .map(|(i, item)| item.profit as i64 + i as i64 * 2 + 1)
        .collect::<Vec<_>>();

    let other = other
        .iter()
        .map(|item| item.profit as i64)
        .collect::<Vec<_>>();

    let mut result = 0i64;
    let mut category_count = k.min(top.len());
    for item in top.iter().take(category_count) {
        result += *item;
    }

    let start = k.max(top.len()) - top.len();
    for item in other.iter().take(start) {
        result += *item;
    }

    let mut max_result = result;
    for item in other.iter().skip(start) {
        if category_count == 0 {
            break;
        }
        result -= top[category_count - 1];
        result += *item;
        category_count -= 1;
        max_result = max_result.max(result);
    }

    max_result
}

#[derive(Debug)]
struct Item {
    profit: i32,
    elegance: i32,
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let items = common::parse_grid::<i32>("[[3,2],[5,1],[10,1]]").unwrap();
        let k = 2;
        let result = find_maximum_elegance(items, k);
        assert_eq!(result, 17);
    }

    #[test]
    fn test2() {
        let items = common::parse_grid::<i32>("[[3,1],[3,1],[2,2],[5,3]]").unwrap();
        let k = 3;
        let result = find_maximum_elegance(items, k);
        assert_eq!(result, 19);
    }

    #[test]
    fn test3() {
        let items = common::parse_grid::<i32>("[[1,1],[2,1],[3,1]]").unwrap();
        let k = 3;
        let result = find_maximum_elegance(items, k);
        assert_eq!(result, 7);
    }

    #[test]
    fn test4() {
        let items = common::parse_grid::<i32>(
            "[[6,4],[14,4],[14,6],[15,6],[2,10],[4,1],[3,2],[13,4],[3,3],[12,5]]",
        )
        .unwrap();
        let k = 6;
        let result = find_maximum_elegance(items, k);
        assert_eq!(result, 88);
    }

    #[test]
    fn test5() {
        let items =
            common::parse_grid::<i32>("[[9,1],[8,1],[8,1],[8,1],[2,2],[2,3],[2,4]]").unwrap();
        let k = 4;
        let result = find_maximum_elegance(items, k);
        assert_eq!(result, 34);
    }
}
