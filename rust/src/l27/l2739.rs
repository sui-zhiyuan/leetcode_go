use std::cmp;

pub fn distance_traveled(mut main_tank: i32, additional_tank: i32) -> i32 {
    let dis = 1;
    main_tank -= 1;
    let max_transfer = main_tank / 4;
    let max_transfer = cmp::min(max_transfer, additional_tank);
    (dis + main_tank + max_transfer) * 10
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(distance_traveled(5, 10), 60);
        assert_eq!(distance_traveled(1, 2), 10);
    }
}