impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        if chars[0] == '?' {
            chars[0] = if chars[1] == '?' || chars[1] < '2' {
                '1'
            } else {
                '0'
            };
        }

        if chars[1] == '?' {
            chars[1] = if chars[0] == '1'  {
                '1'
            } else {
                '9'
            };
        }

        if chars[3] == '?' {
            chars[3] = '5';
        }

        if chars[4] == '?' {
            chars[4] = '9';
        }

        chars.into_iter().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_latest_time("1?:?4".to_string()), "11:54".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_latest_time("?3:12".to_string()), "03:12".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_latest_time("?2:2?".to_string()), "02:29".to_string());
    }
}