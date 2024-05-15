pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut num_pos = vec![0; nums.len()];
    for (i, &num) in nums.iter().enumerate() {
        num_pos[num as usize] = i as isize;
    }
    let mut circles = Vec::new();
    let mut first_not_find = 0;
    while first_not_find < num_pos.len() {
        let mut curr = first_not_find;
        circles.push(Vec::new());
        let len1 = circles.len();
        let cc = &mut circles[len1 - 1];
        while num_pos[curr] >= 0 {
            cc.push(curr);
            let next = num_pos[curr] as usize;
            num_pos[curr] = -1;
            curr = next;
        }

        while first_not_find < num_pos.len() && num_pos[first_not_find] < 0 {
            first_not_find += 1;
        }
    }

    // for v in circles.iter_mut(){
    //     v.sort()
    // }
    // circles.sort();
    // dbg!(&circles);

    let mut los = vec![(0usize, 0usize); n];
    for (i, c) in circles.iter().enumerate() {
        for (j, &v) in c.iter().enumerate() {
            los[v] = (i, j);
        }
    }

    let mut used = vec![false; circles.len()];
    let mut result = circles[0].clone();
    let mut score = 0;
    used[0] = true;

    'outer: for _ in 1..circles.len() {
        let mut bad_los = (0, 0, 0);
        for i in 0..result.len() {
            let mut min_next = if i < result.len() - 1 {
                result[i + 1]
            } else {
                usize::MAX
            };
            let mut min_los = (0, 0);

            for z in [-1, 1] {
                let p = result[i] as isize + z;
                if p < 0 || p as usize >= n {
                    continue;
                }

                let v_los = los[p as usize];
                if !used[v_los.0] {
                    let t_circle = &circles[v_los.0];
                    let t_value = t_circle[(v_los.1 + 1) % t_circle.len()];
                    if t_value < min_next {
                        min_next = t_value;
                        min_los = v_los;
                        min_los.1 = (min_los.1+1) %  t_circle.len();
                    } else {
                        bad_los = (v_los.0, (v_los.1 + 1)%  t_circle.len(), i);
                    }
                }
            }

            if min_los.0 != 0 {
                let mut new_result = [
                    // 需要优化为链表
                    &result[..=i],
                    &circles[min_los.0][min_los.1..],
                    &circles[min_los.0][..min_los.1],
                    &result[i + 1..],
                ]
                .concat();
                used[min_los.0] = true;
                score += 2;

                // if (i ==  result.len() - 1 || 上一个合并边缘) && circles[min_los.0].len() == 1 
                if  circles[min_los.0].len() == 1 || i ==  result.len() - 1  {
                    for i in 1..=result.len() {
                        let check_result =
                            [&result[..i], &circles[min_los.0], &result[i..]].concat();

                        let mut check_score = 0;
                        for k in 0..check_result.len() {
                            let v1 = check_result[k];
                            let v2 = nums[check_result[(k + 1) % check_result.len()]] as usize;
                            check_score += v1.max(v2) - v1.min(v2);
                        }

                        if check_score > score {
                            continue;
                        }

                        if check_result < new_result {
                            new_result = check_result;
                        }
                    }
                }

                result = new_result;
                continue 'outer;
            }
        }
        result = [
            &result[..=bad_los.2],
            &circles[bad_los.0][bad_los.1..],
            &circles[bad_los.0][..bad_los.1],
            &result[bad_los.2 + 1..],
        ]
        .concat();
        used[bad_los.0] = true;
        score += 2;
    }
    result.iter().map(|&v| v as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 0, 2];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0, 1, 2]);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 2, 1];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0, 2, 1]);
    }

    #[test]
    fn test3() {
        let nums = vec![4, 2, 0, 3, 1];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0, 2, 1, 4, 3]);
    }

    #[test]
    fn test4() {
        let nums = vec![3, 2, 1, 4, 0];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0, 2, 1, 4, 3]);
    }

    #[test]
    fn test5() {
        let nums = vec![4, 5, 0, 1, 2, 3];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0, 2, 3, 5, 1, 4]);
    }

    #[test]
    fn test6() {
        let nums = vec![0, 2, 1, 3];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0, 2, 3, 1]);
    }

    #[test]
    fn test7() {
        let nums = vec![0, 3, 2, 1];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0,2,3,1]);
    }

    #[test]
    fn test8() {
        let nums = vec![0,4,2,1,3];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0,2,3,4,1]);
    }

    #[test]
    fn test9() {
        let nums = vec![0,4,3,2,1];
        let res = find_permutation(nums);
        assert_eq!(res, vec![0,3,2,4,1]);
    }
}
