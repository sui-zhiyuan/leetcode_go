pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let mut cost_by_day = vec![i32::MAX; days.len()+1];

    let [cost1 , cost7, cost30] = costs[..] else {
        panic!("costs must have 3 elements");
    };

    let costs = [Cost{days: 1, cost: cost1}, Cost{days: 7, cost: cost7}, Cost{days: 30, cost: cost30}];

    cost_by_day[0] = 0;
    for i in 0..=days.len() {
        for c in costs.iter(){
            let mut j = i +1 ;
            while j <= days.len() && days[j-1] < days[i] + c.days {
                cost_by_day[j] = cost_by_day[j].min(cost_by_day[i] + c.cost);
                j += 1;
            }
        }
    }

    cost_by_day[days.len()]
}

struct Cost {
    days : i32,
    cost : i32
}