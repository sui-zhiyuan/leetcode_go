use std::mem;

impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let max = nums.iter().max().copied().unwrap();
        let mut state1 = vec![None; nums.len() + 1];
        let mut state2 = vec![None; nums.len() + 1];
        state1[0] = Some(0);
        let mut curr = &mut state1;
        let mut next = &mut state2;
        for v in and_values {
            let mut i = 1;
            'outer: while i < next.len() {
                loop {
                    if i >= next.len() {
                        break 'outer;
                    }
                    if nums[i - 1] & v == v {
                        break;
                    }
                    i += 1;
                }
                // dbg!(("skip", i));

                let mut require = i;
                let mut bc = BitCount::new(max);
                loop {
                    if i >= next.len() || nums[i - 1] & v != v {
                        continue 'outer;
                    }
                    bc.add(nums[i - 1]);
                    if bc.fit(v) {
                        break;
                    }
                    i += 1;
                }
                // dbg!((i, require));

                let mut min = curr[require - 1];
                loop {
                    while bc.fit(v) {
                        if curr[require - 1].is_some() && (min.is_none() || min > curr[require - 1])
                        {
                            min = curr[require - 1];
                        }
                        bc.remove(nums[require - 1]);
                        require += 1;
                    }
                    next[i] = min.map(|x| x + nums[i - 1]);
                    i += 1;
                    if i >= next.len() {
                        break 'outer;
                    }
                    if nums[i - 1] & v != v {
                        break;
                    }
                    bc.add(nums[i - 1]);
                }
            }
            curr.fill(None);
            (curr, next) = (next, curr);
            // dbg!(&curr);
        }
        curr[nums.len()].unwrap_or(-1)
    }
}

struct BitCount(Vec<i32>, i32);

impl BitCount {
    fn new(max: i32) -> Self {
        let len = mem::size_of::<i32>() * 8 - max.leading_zeros() as usize;
        Self(vec![0; len], 0)
    }

    fn add(&mut self, v: i32) {
        let mut v = v;
        for i in 0..self.0.len() {
            self.0[i] += v & 1;
            v >>= 1;
        }
        self.1 += 1;
    }

    fn remove(&mut self, v: i32) {
        let mut v = v;
        for i in 0..self.0.len() {
            self.0[i] -= v & 1;
            v >>= 1;
        }
        self.1 -= 1;
    }

    fn fit(&self, v: i32) -> bool {
        if self.1 == 0 {
            return false;
        }
        let mut v = v;
        for i in 0..self.0.len() {
            if v & 1 == 1 && self.0[i] != self.1 {
                return false;
            }
            if v & 1 == 0 && self.0[i] >= self.1 {
                return false;
            }
            v >>= 1;
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]),
            12
        );
    }
}
