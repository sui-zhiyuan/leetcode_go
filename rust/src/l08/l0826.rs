pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut jobs = difficulty
        .iter()
        .zip(profit.iter())
        .map(|(d, j)| Job {
            difficulty: *d,
            profit: *j,
        })
        .collect::<Vec<Job>>();

    jobs.sort_by_key(|j| j.difficulty);

    let mut max_profit = jobs[0].profit;
    for job in jobs.iter_mut().skip(1) {
        max_profit = max_profit.max(job.profit);
        job.profit = max_profit;
    }

    let mut all_profit = 0;
    for w in worker {
        let fist_miss = binary_search(0, jobs.len(), |i| jobs[i].difficulty > w);
        if fist_miss > 0{
            all_profit += jobs[fist_miss - 1].profit;
        }
    }

    all_profit
}

fn binary_search(begin:usize , end:usize, mut f: impl FnMut(usize) -> bool) -> usize {
    let mut begin = begin;
    let mut end = end;
    while begin < end -1{
        let mid = begin + (end - begin) / 2;
        if f(mid) {
            end = mid;
        } else {
            begin = mid;
        }
    }
    if f(begin) {
        begin
    } else {
        end
    }
}   

struct Job {
    difficulty: i32,
    profit: i32,
}
