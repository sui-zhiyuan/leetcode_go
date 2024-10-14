use crate::common::binary_search;

pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let mut tries = n;

    for k in 1..=k {
        tries = binary_search(0, tries, |v| egg_drop_count(k, v) >= n);
    }

    tries
}

fn egg_drop_count(egg_remain: i32 , drop_remain:i32) -> i32{
    if drop_remain == 0 {
        return 0;
    }
    if egg_remain == 1 {
        return drop_remain;
    }

    let mut result = 0;
    for drop in 1..=drop_remain{
        result += egg_drop_count(egg_remain - 1, drop_remain - drop);
        result += 1
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let k = 1;
        let n = 2;
        let result = super_egg_drop(k, n);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2(){
        let k = 2;
        let n = 6;
        let result = super_egg_drop(k, n);
        assert_eq!(result, 3);
    }

    #[test]
    fn test3(){
        let k = 3;
        let n = 14;
        let result = super_egg_drop(k, n);
        assert_eq!(result, 4);
    }

    #[test]
    fn test4(){
        let k = 2;
        let n = 1;
        let result = super_egg_drop(k, n);
        assert_eq!(result, 1);
    }
}