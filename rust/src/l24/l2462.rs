use std::cmp;

pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let mut left = Heap {
        start: 0,
        len: candidates as usize,
    };
    let mut right = Heap {
        start: costs.len() - candidates as usize,
        len: candidates as usize,
    };

    let mut rest = if candidates as usize * 2 >= costs.len() {
        left = Heap {
            start: 0,
            len: costs.len(),
        };
        right = Heap { start: 0, len: 0 };
        (0, 0)
    } else {
        (candidates as usize, costs.len() - candidates as usize)
    };

    build_heap(&mut costs, &mut left);
    build_heap(&mut costs, &mut right);
    let mut lv = get_min_value(&costs, &left);
    let mut rv = get_min_value(&costs, &right);
    let mut result = 0;
    for _ in 0..k as usize {
        if lv.map(cmp::Reverse) >= rv.map(cmp::Reverse) {
            result += lv.unwrap() as i64;
            lv = get_rest_left(&mut costs, &mut rest);
            take_value(&mut costs, &mut left, lv);
            lv = get_min_value(&costs, &left);
        } else {
            result += rv.unwrap() as i64;
            rv = get_rest_right(&mut costs, &mut rest);
            take_value(&mut costs, &mut right, rv);
            rv = get_min_value(&costs, &right);
        }
    }

    result
}

fn build_heap(cost: &mut [i32], heap: &mut Heap) {
    let heap = &mut cost[heap.start..heap.start + heap.len];
    for i in (0..heap.len()).rev() {
        adjust(heap, i)
    }
}

fn get_rest_left(cost: &mut [i32], rest: &mut (usize, usize)) -> Option<i32> {
    if rest.0 < rest.1 {
        rest.0 += 1;
        Some(cost[rest.0 - 1])
    } else {
        None
    }
}

fn get_rest_right(cost: &mut [i32], rest: &mut (usize, usize)) -> Option<i32> {
    if rest.0 < rest.1 {
        rest.1 -= 1;
        Some(cost[rest.1])
    } else {
        None
    }
}

fn get_min_value(cost: &[i32], heap: &Heap) -> Option<i32> {
    if heap.len == 0 {
        return None;
    }
    Some(cost[heap.start])
}

fn take_value(cost: &mut [i32], heap: &mut Heap, replace: Option<i32>) {
    if heap.len == 0 {
        return;
    }
    let mut vh = &mut cost[heap.start..heap.start + heap.len];
    if let Some(v) = replace {
        vh[0] = v;
    } else {
        let new;
        (new, vh) = vh.split_last_mut().unwrap();
        if !vh.is_empty() {
            vh[0] = *new
        }
    }
    adjust(vh , 0);
    heap.len = vh.len();
}

fn adjust(heap: &mut[i32] ,mut curr:usize){
    while left(curr) < heap.len() {
        let mut next = left(curr);
        if right(curr) < heap.len() && heap[right(curr)] < heap[next] {
            next = right(curr);
        }

        if heap[curr] <= heap[next] {
            break;
        }
        heap.swap(curr, next);
        curr = next;
    }
}

fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

struct Heap {
    start: usize,
    len: usize,
}

#[cfg(test)]
mod test{
    use crate::common;
    use super::*;

    #[test]
    fn test1(){
        let cost = common::parser_vec::<i32>("[3,2,7,7,1,2]").unwrap();
        assert_eq!(3, total_cost(cost, 2, 2));
    }

    #[test]
    fn test2(){
        let cost = common::parser_vec::<i32>("[17,12,10,2,7,2,11,20,8]").unwrap();
        assert_eq!(11, total_cost(cost, 3, 4));
    }

    #[test]
    fn test3(){
        let cost = common::parser_vec::<i32>("[1,2,4,1]").unwrap();
        assert_eq!(4, total_cost(cost, 3, 3));
    }

    #[test]
    fn test4(){
        let cost = common::parser_vec::<i32>("[50,80,34,9,86,20,67,94,65,82,40,79,74,92,84,37,19,16,85,20,79,25,89,55,67,84,3,79,38,16,44,2,54,58]").unwrap();
        assert_eq!(95, total_cost(cost, 7, 12));
    }

    #[test]
    fn test5(){
        let cost = common::parser_vec::<i32>("[57,33,26,76,14,67,24,90,72,37,30]").unwrap();
        assert_eq!(526, total_cost(cost, 11, 2));
    }

    #[test]
    fn test6(){
        let cost = common::parser_vec::<i32>("[10,1,11,10]").unwrap();
        assert_eq!(11, total_cost(cost, 2, 1));
    }
}