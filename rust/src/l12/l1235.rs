use std::cmp;

pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut works = start_time
        .into_iter()
        .zip(end_time)
        .zip(profit)
        .map(|((s, e), p)| (s, e, p))
        .collect::<Vec<(i32, i32, i32)>>();

    works.sort_by_key(|(_, e, _)| *e);
    let mut max = 0;
    let mut profit = Vec::<(i32, i32)>::with_capacity(works.len() + 1);
    profit.push((0, 0));

    for (s, e, p) in works {
        let &(_, ps) = binary_search(&profit, |&(t, _)| t > s);
        let mut last = profit.last_mut().unwrap();
        if last.0 != e {
            assert!(last.0 < e);
            let p_last = last.1;
            profit.push((e, p_last));
            last = profit.last_mut().unwrap();
        }
        let pe = &mut last.1;
        *pe = cmp::max(*pe, ps + p);
        max = cmp::max(max, *pe);
    }

    max
}

fn binary_search<T, F>(vec: &[T], f: F) -> &T
where
    F: Fn(&T) -> bool,
{
    let mut left = 0;
    let mut right = vec.len();
    while left < right - 1 {
        let mid = left + (right - left) / 2;
        if f(&vec[mid]) {
            right = mid;
        } else {
            left = mid;
        }
    }
    &vec[left]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        assert_eq!(job_scheduling(start_time, end_time, profit), 120);
    }

    #[test]
    fn test2() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        assert_eq!(job_scheduling(start_time, end_time, profit), 150);
    }

    #[test]
    fn test3() {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        assert_eq!(job_scheduling(start_time, end_time, profit), 6);
    }
}
