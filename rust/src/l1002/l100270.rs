impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let vs = s.chars().map(|v| v as i32).collect::<Vec<_>>();
        let mut score = 0;

        for i in 0..(vs.len() - 1) {
            score += (vs[i] - vs[i + 1]).abs();
        }
        score
    }
}

pub struct Solution();


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::score_of_string("hello".to_string()), 13);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::score_of_string("abc".to_string()), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::score_of_string("aab".to_string()), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::score_of_string("a".to_string()), 0);
    }
}
