use std::collections::VecDeque;

pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
    let robots = charge_times
        .iter()
        .zip(running_costs.iter())
        .map(|(c, r)| Robot {
            charge_time: *c,
            running_cost: *r,
        })
        .collect::<Vec<_>>();

    let mut monotonic_stack: VecDeque<usize> = VecDeque::new();
    let mut sum_cost: i64 = 0;
    let mut start = 0;
    let mut end = 0;
    let mut result = 0;
    while end < robots.len() {
        sum_cost += robots[end].running_cost as i64;
        while let Some(&i) = monotonic_stack.back(){
            if robots[i].charge_time > robots[end].charge_time {
                break;
            }
            monotonic_stack.pop_back();
        }
        monotonic_stack.push_back(end);
        end += 1;

        while let Some(&i) = monotonic_stack.front(){
            let total = robots[i].charge_time as i64 + sum_cost * (end - start) as i64;
            if total <= budget{
                break;
            }
            sum_cost -= robots[start].running_cost as i64;
            if let Some(&i) = monotonic_stack.front(){
                if i == start{
                    monotonic_stack.pop_front();
                }
            }
            start += 1;
        }

        result = result.max(end - start);
    }

    result as i32
}

struct Robot {
    charge_time: i32,
    running_cost: i32,
}
