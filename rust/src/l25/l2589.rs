use std::cmp;

pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {
    let mut tasks = tasks
        .into_iter()
        .map(|task| Task {
            start: task[0] as usize,
            end: task[1] as usize,
            duration: task[2] as usize,
        })
        .collect::<Vec<_>>();

    tasks.sort_by_key(|t| (cmp::Reverse(t.start) , cmp::Reverse(t.duration)));
    let ends = tasks.iter().map(|t| t.end).max();
    let mut used = vec![false; ends.unwrap_or(0) + 1];

    let mut total_duration = 0;
    for i in 0..tasks.len(){
        let Task{start , mut duration ,.. } = tasks[i];
        let mut j = start;
        while duration > 0 {
            if used[j] {
                j += 1;
                continue;
            }
            used[j] = true;
            total_duration +=1;
            for t in tasks.iter_mut().skip(i+1){
                if t.start <= j && j <= t.end  && t.duration >= 1{
                    t.duration -=1;
                }
            }

            duration -=1;
            j +=1;
        }
    }
    total_duration
}

#[derive(Debug)]
struct Task {
    start: usize,
    end: usize,
    duration: usize,
}

#[cfg(test)]
mod test {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let tasks = common::parse_grid::<i32>("[[2,3,1],[4,5,1],[1,5,2]]").unwrap();
        let res = 2;
        assert_eq!(find_minimum_time(tasks), res);
    }

    #[test]
    fn test2() {
        let tasks = common::parse_grid::<i32>("[[1,3,2],[2,5,3],[5,6,2]]").unwrap();
        let res = 4;
        assert_eq!(find_minimum_time(tasks), res);
    }

    #[test]
    fn test3() {
        let tasks = common::parse_grid::<i32>("[[10,16,3],[10,20,5],[1,12,4],[8,11,2]]").unwrap();
        let res = 6;
        assert_eq!(find_minimum_time(tasks), res);
    }

    #[test]
    fn test4() {
        let tasks = common::parse_grid::<i32>("[[12,15,4],[67,104,2],[26,26,1],[68,68,1],[77,103,3],[60,89,9],[46,46,1],[70,86,2],[45,45,1],[24,24,1],[48,70,3],[20,20,1],[37,38,2],[65,65,1],[57,65,5],[23,23,1],[63,67,5],[36,55,3],[6,6,1],[65,80,4],[17,22,6],[53,75,3],[61,61,1],[52,52,1],[81,81,1],[52,69,8],[38,38,1],[78,78,1],[22,45,1],[61,77,5],[21,24,1],[37,37,1],[49,66,4],[74,82,9],[11,11,1],[57,57,1],[30,30,1],[15,15,1],[4,11,7],[77,77,1],[71,71,1],[48,48,1],[19,33,7],[15,16,2],[13,13,1],[25,35,8],[46,50,5],[64,87,2],[1,1,1],[80,95,4],[69,79,6],[36,36,1],[69,69,1],[1,1,1],[31,46,8],[22,22,1],[26,36,3],[33,33,1],[55,60,6],[47,49,3],[11,18,5],[10,23,2],[66,66,1],[56,101,4],[28,37,7],[39,39,1],[47,47,1],[34,44,3],[76,76,1],[3,7,5]]").unwrap();
        let res = 66;
        assert_eq!(find_minimum_time(tasks), res);
    }
}