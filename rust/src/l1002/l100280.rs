impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        if points.is_empty() {
            return 0;
        }

        let mut vx = points.iter().map(|v| v[0]).collect::<Vec<_>>();
        vx.sort();
        let vx = vx;
        // dbg!(&vx);

        let mut res = 1;
        let mut end = vx[0] + w;
        for v in vx.iter() {
            if v <= &end {
                continue;
            }

            res += 1;
            end = v + w;
            // dbg!(v, end , res);
        }

        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let points = vec![
            vec![2, 1],
            vec![1, 0],
            vec![1, 4],
            vec![1, 8],
            vec![3, 5],
            vec![4, 6],
        ];
        assert_eq!(Solution::min_rectangles_to_cover_points(points, 1), 2);
    }

    #[test]
    fn test_1() {
        let points = vec![
            vec![0, 0],
            vec![1, 0],
            vec![2, 4],
            vec![3, 8],
            vec![4, 5],
            vec![5, 6],
            vec![6, 6],
        ];
        assert_eq!(Solution::min_rectangles_to_cover_points(points, 2), 3);
    }
}
