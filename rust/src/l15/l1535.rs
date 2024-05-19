pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    if k >= arr.len() - 1 {
        return *arr.iter().max().unwrap();
    }

    let mut win = 0;
    let mut next = 1;
    while win < k {
        if arr[0] < arr[next] {
            arr.swap(0, next);
            win = 0;
        }

        win += 1;
        next += 1;
        if next == arr.len() {
            next = 1;
        }
    }
    
    arr[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![2, 1, 3, 5, 4, 6, 7];
        let k = 2;
        let res = 5;
        assert_eq!(get_winner(arr, k), res);
    }

    #[test]
    fn test1() {
        let arr = vec![3, 2, 1];
        let k = 10;
        let res = 3;
        assert_eq!(get_winner(arr, k), res);
    }

    #[test]
    fn test2() {
        let arr = vec![1, 9, 8, 2, 3, 7, 6, 4, 5];
        let k = 7;
        let res = 9;
        assert_eq!(get_winner(arr, k), res);
    }
}
