pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let candies = candies as i64;
    let num_people = num_people as i64;
    let give = (((8 * candies + 1) as f64).sqrt() - 3f64) / 2f64;
    let give = give.trunc() as i64;

    let sum_whole = (give + 1) * (give + 2) / 2;
    let rest = candies - sum_whole;
    let round = give / num_people;
    let half_round_give = give % num_people;

    let mut result = vec![0; num_people as usize];
    for (i, v) in result.iter_mut().enumerate() {
        let i = i as i64;
        let t = round + if i <= half_round_give { 1 } else { 0 };
        let mut v1 = t * (i + 1) + num_people * (t - 1) * t / 2;
        if i == (half_round_give + 1) % num_people {
            v1 += rest;
        }
        *v = v1 as i32;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let candies = 7;
        let num_people = 4;
        let res = vec![1, 2, 3, 1];
        assert_eq!(distribute_candies(candies, num_people), res);
    }

    #[test]
    fn test2() {
        let candies = 10;
        let num_people = 3;
        let res = vec![5, 2, 3];
        assert_eq!(distribute_candies(candies, num_people), res);
    }

    #[test]
    fn test3() {
        let candies = 60;
        let num_people = 4;
        let res = vec![15, 18, 15, 12];
        assert_eq!(distribute_candies(candies, num_people), res);
    }
}
