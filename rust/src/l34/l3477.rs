pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let mut baskets = baskets;
    let mut count = 0;
    for f in fruits.iter().copied() {
        let mut basket = None;

        for (i, &b) in baskets.iter().enumerate() {
            if b >= f {
                basket = Some(i);
                break;
            }
        }

        if let Some(basket) = basket {
            baskets[basket] = -1;
            count += 1;
        }
    }

    fruits.len() as i32 - count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, num_of_unplaced_fruits(vec![1, 4], vec![8, 1]))
    }
}
