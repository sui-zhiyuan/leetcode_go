pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut costs = vec![i32::MAX; n + 2];
    costs[0] = 0;

    for (c, t) in cost.into_iter().zip(time.into_iter().map(|t| (t + 1) as usize)) {
        for i in (0..costs.len()).rev() {
            if costs[i] == i32::MAX {
                continue;
            }

            let mut next = i + t;
            if next > n + 1 {
                next = n + 1;
            }
            
            costs[next] = costs[next].min(costs[i] + c);
        }
    }
    
    costs[n].min(costs[n + 1])
}
