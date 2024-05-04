use std::cmp;

pub fn trap(height: Vec<i32>) -> i32 {
    let (mut left, mut max_left) = (0, height[0]);
    let (mut right, mut max_right) = (height.len() - 1, height[height.len() - 1]);

    let mut ans =0;
    while left < right {
        if max_left < max_right {
            ans += max_left - height[left];
            left += 1;
            max_left = cmp::max(max_left, height[left]);
        } else {
            ans += max_right - height[right];
            right -= 1;
            max_right = cmp::max(max_right, height[right]);
        }
    }
    ans += cmp::min(max_left, max_right) - height[left];

    ans
}

// debug and test

pub fn trap2(height: Vec<i32>) -> i32 {
    let mut saved = Vec::<(i32, usize)>::new();

    let mut ans = 0;
    'outer: for (i, v) in height.into_iter().enumerate() {
        let mut last = 0;
        loop {
            if saved.is_empty() {
                saved.push((v, i));
                continue 'outer;
            }
            let (top, idx) = saved[saved.len() - 1];
            if top > v {
                ans += (i - idx - 1) as i32 * (v - last);
                saved.push((v, i));
                continue 'outer;
            }
            _ = saved.pop();
            ans += (i - idx - 1) as i32 * (top - last);
            last = top;
        }
    }

    ans
}

pub fn trap1(height: Vec<i32>) -> i32 {
    let len = height.len();
    let mut max = vec![i32::MAX; len];
    max[0] = height[0];
    max[len - 1] = height[len - 1];
    for (i, v) in height.iter().enumerate().skip(1).take(len - 2) {
        max[i] = cmp::max(*v, cmp::min(max[i - 1], max[i]));
    }

    for (i, v) in height.iter().enumerate().skip(1).take(len - 2).rev() {
        max[i] = cmp::max(*v, cmp::min(max[i + 1], max[i]));
    }

    height.iter().zip(max).map(|(h, m)| m - h).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(6, trap(height));
    }

    #[test]
    fn test2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(9, trap(height));
    }
}
