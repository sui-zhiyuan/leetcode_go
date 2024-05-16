use std::cmp;

pub fn number_of_weeks(mut milestones: Vec<i32>) -> i64 {
    milestones.sort_by_key(|&x| cmp::Reverse(x));
    let max = milestones[0] as i64;
    let rest = milestones.iter().skip(1).map(|&x| x as i64).sum::<i64>();
    if max > rest + 1 {
        rest * 2 + 1
    } else {
        max + rest
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1(){
        let milestones = vec![1,2,3];
        assert_eq!(number_of_weeks(milestones), 6);
    }
}