use std::mem;

pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
    let mut curr: Vec<Option<i64>> = vec![None ; 5];
    let mut next: Vec<Option<i64>> = vec![None ; 5];

    curr[0] = Some(0);
    for v in b {
        next[0] = Some(0);
        for i in 1..=4 {
            next[i] = curr[i].max(curr[i-1].map(|c| c+ a[i-1] as i64 * v as i64) );
        }

        mem::swap(&mut curr, &mut next);
    }

    curr[4].unwrap()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(max_score(vec![3,2,5,6], vec![2,-6,4,-5,-3,2,-7]), 26);
    }
}