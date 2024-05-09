pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    let mut steps = 0;
    let mut remain = 0;

    for (i,&v) in plants.iter().enumerate() {
        if remain < v {
            steps += 2* i;
            remain = capacity;
        }

        steps += 1;
        remain -= v;
    }

    steps as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let plants = vec![2,2,3,3];
        let capacity = 5;
        assert_eq!(14, watering_plants(plants, capacity));
    }

    #[test]
    fn test2() {
        let plants = vec![1,1,1,4,2,3];
        let capacity = 4;
        assert_eq!(30, watering_plants(plants, capacity));
    }

    #[test]
    fn test3() {
        let plants = vec![7,7,7,7,7,7,7];
        let capacity = 8;
        assert_eq!(49, watering_plants(plants, capacity));
    }
}