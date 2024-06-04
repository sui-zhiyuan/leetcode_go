pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
    let sum = sweetness.iter().sum::<i32>();
    let max = sum / (k + 1);
    binary_search(0, max+2, |v|{
        if v == 0 {
            return false;
        }
        let mut count = 0;
        let mut c_cum = 0;
        for &s in &sweetness {
            c_cum += s;
            if c_cum >= v {
                count += 1;
                c_cum = 0;
            }
        }
        count <= k
    }) -1
}

fn binary_search(start: i32, end: i32, mut check: impl FnMut(i32) -> bool) -> i32 {
    let mut start = start;
    let mut end = end;
    while start < end - 1 {
        let mid = start + (end - start) / 2;
        if check(mid) {
            end = mid;
        } else {
            start = mid;
        }
    }
    if check(start) {
        start
    } else {
        end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let sweetness = vec![1,2,3,4,5,6,7,8,9];
        let k = 5;
        let res = 6;
        assert_eq!(maximize_sweetness(sweetness, k), res);
    }

    #[test]
    fn test2() {
        let sweetness = vec![5,6,7,8,9,1,2,3,4];
        let k = 8;
        let res = 1;
        assert_eq!(maximize_sweetness(sweetness, k), res);
    }

    #[test]
    fn test3() {
        let sweetness = vec![1,2,2,1,2,2,1,2,2];
        let k = 2;
        let res = 5;
        assert_eq!(maximize_sweetness(sweetness, k), res);
    }
}