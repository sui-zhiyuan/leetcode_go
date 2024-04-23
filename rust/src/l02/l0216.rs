pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    find_combine(9, k.try_into().unwrap(), n)
}

fn find_combine(max: i32, count: usize, sum: i32) -> Vec<Vec<i32>> {
    if count == 1 {
        if sum >= 1 && sum <= max {
            return vec![vec![sum]];
        }
        return Vec::new();
    }
    let mut result = Vec::new();
    for m1 in 1..=max {
        let sum = sum - m1;
        let sub = find_combine(m1 -1, count - 1, sum);
        for mut v in sub.into_iter(){
            v.push(m1);
            result.push(v);
        }
    }
    dbg!(max , count , sum , &result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(combination_sum3(3, 7), vec![vec![1, 2, 4]])
    }

    #[test]
    fn test2() {
        assert_eq!(
            combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        )
    }

    #[test]
    fn test3() {
        assert_eq!(combination_sum3(4, 1), Vec::<Vec<i32>>::new())
    }
}
