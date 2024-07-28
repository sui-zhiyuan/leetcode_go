use std::{collections::HashMap, mem};

pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    nums.sort();
    let mut state = vec![vec![HashMap::<i32,i64>::new(); k+1] ; nums.len()+1];

    for (i , &v) in nums.iter().enumerate(){
        state[i][1].insert(i32::MAX,1);
        for j in 1..k{
            let mut curr = HashMap::<i32,i64>::new();
            mem::swap(&mut state[i][j] , &mut curr);
            for (&key , &value) in curr.iter(){
                for (n , &vn) in nums.iter().enumerate().skip(i+1){
                    let new_key = key.min(vn - v);
                    let count = state[n][j+1].entry(new_key).or_insert(0);
                    *count = mod_add(*count, value);
                }
            }
            mem::swap(&mut state[i][j] , &mut curr);
        }
    }

    let mut result = 0;
    for row in state.iter(){
        for (&key , &value) in row[k].iter(){
            result = mod_add(result , mod_mul(key as i64 , value));
        }
    }

    result as i32
}

const MOD: i64 = 1_000_000_007;

fn mod_add(a: i64, b: i64) -> i64 {
    (a  + b) % MOD
}

fn mod_mul(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![1,2,3,4];
        let k = 3;
        let result = 4;
        assert_eq!(sum_of_powers(nums, k) , result);
    }

    #[test]
    fn test2(){
        let nums = vec![2,2];
        let k = 2;
        let result = 0;
        assert_eq!(sum_of_powers(nums, k) , result);
    }

    #[test]
    fn test3(){
        let nums = vec![4,3,-1];
        let k = 2;
        let result = 10;
        assert_eq!(sum_of_powers(nums, k) , result);
    }
}