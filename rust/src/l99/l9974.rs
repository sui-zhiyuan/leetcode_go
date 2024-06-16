use crate::common::SegmentTree;

pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut st = SegmentTree::<i32, _>::new_empty(nums.len(), |a, b| a + b);

    for (i,&v) in nums.iter().enumerate() {
        if i == 0 || i == nums.len() - 1 {
            continue;
        }
        if v > nums[i-1] && v > nums[i+1] {
            st.set_value(i, 1)
        }
    }

    let mut result = Vec::new();
    for row in queries {
        let op = row[0];
        match op {
            1 => {
                // dbg!(st.values());
                let left = row[1] as usize +1;
                let right = row[2] as usize;
                if left < right {
                    result.push(st.range(left, right))
                } else {
                    result.push(0)
                }
            }
            2=> {
                let index = row[1] as usize;
                let value = row[2];

                nums[index] = value;
                if index >=1 && index <= nums.len() - 2{
                    let new_val = if nums[index] > nums[index-1] && nums[index] > nums[index+1] {
                        1
                    } else {
                        0
                    };
                    st.set_value(index, new_val);
                }

                if index >=2 {
                    let new_val = if nums[index-1] > nums[index-2] && nums[index-1] > nums[index] {
                        1
                    } else {
                        0
                    };
                    st.set_value(index-1, new_val);
                }

                if index <= nums.len() - 3 {
                    let new_val = if nums[index+1] > nums[index] && nums[index+1] > nums[index+2] {
                        1
                    } else {
                        0
                    };
                    st.set_value(index+1, new_val);
                }
            }

            _ => unreachable!("invalid op")
        }
    }
    
    result
}


#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3,1,4,2,5];
        let queries = common::parse_grid::<i32>("[[2,3,4],[1,0,4]]").unwrap();
        let result = vec![0];
        assert_eq!(count_of_peaks(nums, queries), result);
    }

    #[test]
    fn test_2() {
        let nums = vec![4,1,4,2,1,5];
        let queries = common::parse_grid::<i32>("[[2,2,4],[1,0,2],[1,0,4]]").unwrap();
        let result = vec![0,1];
        assert_eq!(count_of_peaks(nums, queries), result);
    }

    #[test]
    fn test_3() {
        let nums = vec![5,4,8,6];
        let queries = common::parse_grid::<i32>("[[1,2,2],[1,1,2],[2,1,6]]").unwrap();
        let result = vec![0,0];
        assert_eq!(count_of_peaks(nums, queries), result);
    }

}