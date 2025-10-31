use std::collections::BinaryHeap;

pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();

    let mut count = 0;
    let mut result = Vec::new();
    for q in queries {
        let dis = q[0].abs() + q[1].abs();
        match count.cmp(&(k-1)){
            std::cmp::Ordering::Less => {
                count += 1;
                heap.push(dis);
                result.push(-1);
            },
            std::cmp::Ordering::Equal => {
                count += 1;
                heap.push(dis);
                result.push(*heap.peek().unwrap());
            },
            std::cmp::Ordering::Greater => {
                if dis < *heap.peek().unwrap() {
                    heap.pop();
                    heap.push(dis);
                }
                result.push(*heap.peek().unwrap());
            }
        }
    }

    result
}

