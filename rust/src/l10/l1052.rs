use std::cmp;

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let satisfied1 = customers
        .iter()
        .zip(grumpy.iter())
        .map(|v| v.0 * (1 - v.1))
        .sum::<i32>();

    let angry = customers
        .iter()
        .zip(grumpy.iter())
        .map(|v| v.0 * v.1)
        .collect::<Vec<i32>>();

    let minutes = usize::try_from(minutes).unwrap();
    let mut satisfied = 0;
    for i in 0..minutes {
        satisfied += angry[i];
    }

    let mut max_satisfied = satisfied;
    for i in minutes..angry.len(){
        satisfied += angry[i] - angry[i - minutes];
        max_satisfied = cmp::max(max_satisfied, satisfied);
    }

    satisfied1 + max_satisfied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3),
            16
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            max_satisfied(vec![4, 10, 10], vec![1, 1, 0], 2),
            24
        )
    }
}