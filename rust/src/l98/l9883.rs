use std::{cmp::Ordering, collections::HashMap};

const MOD: u64 = 1_000_000_007;

use crate::common::binary_search;

pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    if multiplier == 1 {
        return nums;
    }

    let mut nums = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| Number {
            value: v as u64,
            times: 0,
            index: i,
        })
        .collect::<Vec<Number>>();

    nums.sort();
    let multiplier = multiplier as u64;

    let mut k = k as usize;
    let mut curr;
    let mut times = 1;

    loop {
        let minium = nums[0].value;
        curr = binary_search(0, nums.len(), |v| nums[v].value >= minium * multiplier);
        if curr == nums.len() {
            break;
        }
        // let maximum = nums[curr].value;
        // let minium = nums[curr-1].value;
        // times = (maximum as f64 / minium as f64)
        //     .log(multiplier as f64)
        //     .ceil() as u64;
        if k < times as usize * curr {
             break;
        }
        times = 1;

        let t = multiplier.pow(times as u32);
        for v in nums[0..curr].iter_mut() {
            v.value *= t;
            v.value %= MOD;
        }
        k -= times as usize * curr;
        nums.sort();
    }

    if k > 0 {
        let times = (k / curr) as u64;

        for (i , v) in nums[0..curr].iter_mut().enumerate(){
            v.times = times;
            if i < k % curr {
                v.times += 1;
            }
        }
    }

    to_value(&nums, multiplier)
}

#[derive(Debug, PartialEq, Eq)]
struct Number {
    value: u64,
    times: u64,
    index: usize,
}

fn to_value(number: &[Number], multiplier: u64) -> Vec<i32> {
    let mut pc = PowerCache::new(multiplier);
    let mut result = vec![0i32; number.len()];

    for v in number.iter() {
        let p = pc.get_power(v.times);
        result[v.index] = (p * v.value % MOD) as i32;
    }

    result
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        let c = self.value.cmp(&other.value);
        if c != Ordering::Equal {
            return c;
        }
        self.index.cmp(&other.index)
    }
}

struct PowerCache {
    cache: HashMap<u64, u64>,
    pows: Vec<u64>,
}

impl PowerCache {
    fn new(v: u64) -> Self {
        let pows = vec![v];
        Self {
            cache: HashMap::new(),
            pows,
        }
    }

    fn get_power(&mut self, times: u64) -> u64 {
        if let Some(&v) = self.cache.get(&times) {
            return v;
        }

        let o_times = times;

        let mut mul: u64 = 1;
        let mut curr = 0;
        let mut times = times;
        while times > 0 {
            if times & 1 == 1 {
                mul = mul * self.pows[curr] % MOD;
            }
            times >>= 1;
            curr += 1;
            if curr == self.pows.len() {
                self.pows
                    .push(self.pows[curr - 1] * self.pows[curr - 1] % MOD);
            }
        }
        self.cache.insert(o_times, mul);
        mul
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![2,1,3,5,6];
        let k = 5;
        let multiplier = 2;
        let result = vec![8, 4, 6, 5, 6];
        assert_eq!(get_final_state(nums, k, multiplier), result);
    }

    #[test]
    fn test2(){
        let nums = vec![2,1,4];
        let k = 2;
        let multiplier = 3;
        let result = vec![6,3,4];
        assert_eq!(get_final_state(nums, k, multiplier), result);
    }

    #[test]
    fn test3(){
        let nums = vec![5,2,1,3];
        let k = 5;
        let multiplier = 4;
        let result = vec![20,8,16,12];
        assert_eq!(get_final_state(nums, k, multiplier), result);
    }

    #[test]
    fn test4(){
        let nums = vec![1,2,5];
        let k = 1;
        let multiplier = 2;
        let result = vec![2,2,5];
        assert_eq!(get_final_state(nums, k, multiplier), result);
    }
}