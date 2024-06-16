use std::collections::HashMap;

pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let counts = tasks
        .iter()
        .fold(HashMap::<i32, usize>::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    let mut ans = 0;
    for ( _ ,v) in counts {
        if v == 1 {
            return  -1;
        }
        if v % 3 == 0{
            ans += v / 3;
        }
        if v % 3 ==1 {
            ans += (v-4) / 3 + 2;
        }
        if v % 3 == 2 {
            ans += v / 3 + 1;
        }
    }
    ans as i32
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tasks = vec![5,5,5,5];
        let result = 2;
        assert_eq!(minimum_rounds(tasks), result);
    }

    #[test]
    fn test2() {
        let tasks = vec![2,2,3,3,2,4,4,4,4,4];
        let result = 2;
        assert_eq!(minimum_rounds(tasks), result);
    }

}