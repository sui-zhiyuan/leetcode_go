pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut heap = std::collections::BinaryHeap::new();

    let mut count = 0;
    let mut result = Vec::new();
    for q in queries {
        let dis = q[0].abs() + q[1].abs();
        if count < k-1{
            count += 1;
            heap.push(dis);
            result.push(-1);
        } else if count == k -1 {
            count += 1;
            heap.push(dis);
            result.push(*heap.peek().unwrap());
        }
        else {
            if dis < *heap.peek().unwrap() {
                heap.pop();
                heap.push(dis);
            }
            result.push(*heap.peek().unwrap());
        }
    }

    result
}

