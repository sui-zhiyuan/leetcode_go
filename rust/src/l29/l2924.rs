impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut heads = edges
            .into_iter()
            .map(|v| v[1])
            .fold(vec![false; n as usize], |mut acc, v| {
                acc[v as usize] = true;
                acc
            })
            .into_iter()
            .enumerate()
            .filter(|(_, v)| !v);
        let v = if let Some((i, _)) = heads.next() {
            i
        } else {
            return -1;
        };
        if heads.next().is_some() {
            return -1;
        }
        v as i32
    }
}

pub struct Solution();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::find_champion(3, edges), 0);
    }

    #[test]
    fn test_1() {
        let edges = vec![vec![0, 2], vec![1, 2], vec![1, 3]];
        assert_eq!(Solution::find_champion(4, edges), -1);
    }
}
