// cspell:ignore chebyshev

use std::collections::HashSet;

pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
    let points = points
        .into_iter()
        .map(|point| Point {
            x: point[0],
            y: point[1],
        })
        .map(|v| v.to_chebyshev_point())
        .collect::<Vec<Point>>();

    let mut edges: [Option<usize>; 8] = [None; 8];

    for (i, point) in points.iter().enumerate() {
        if edges[0].is_none() || point.x < points[edges[0].unwrap()].x {
            edges[1] = edges[0];
            edges[0] = Some(i);
        } else if edges[1].is_none() || point.x < points[edges[1].unwrap()].x {
            edges[1] = Some(i);
        }

        if edges[2].is_none() || point.y < points[edges[2].unwrap()].y {
            edges[3] = edges[2];
            edges[2] = Some(i);
        } else if edges[3].is_none() || point.y < points[edges[3].unwrap()].y {
            edges[3] = Some(i);
        }

        if edges[4].is_none() || point.x > points[edges[4].unwrap()].x {
            edges[5] = edges[4];
            edges[4] = Some(i);
        } else if edges[5].is_none() || point.x > points[edges[5].unwrap()].x {
            edges[5] = Some(i);
        }

        if edges[6].is_none() || point.y > points[edges[6].unwrap()].y {
            edges[7] = edges[6];
            edges[6] = Some(i);
        } else if edges[7].is_none() || point.y > points[edges[7].unwrap()].y {
            edges[7] = Some(i);
        }
    }

    let edges = edges.into_iter().flatten().collect::<HashSet<_>>();
    let edges = edges.into_iter().map(|i| points[i]).collect::<Vec<_>>();
    let n = edges.len();
    let mut min_max = i32::MAX;
    for i in 0..n {
        let mut max = 0;
        for j in 0..n {
            if j == i {
                continue;
            }
            for k in j + 1..n {
                if k == i {
                    continue;
                }

                max = max.max(edges[j].chebyshev_distance(&edges[k]));
            }
        }
        min_max = min_max.min(max);
    }

    min_max
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn to_chebyshev_point(self) -> Point {
        Point {
            x: self.x + self.y,
            y: self.x - self.y,
        }
    }

    fn chebyshev_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let points = [[3, 10], [5, 15], [10, 2], [4, 4]];
        let points = points.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        assert_eq!(minimum_distance(points), 12);
    }

    #[test]
    fn test3() {
        let points = [[3, 2], [3, 9], [7, 10], [4, 4], [8, 10], [2, 7]];
        let points = points.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        assert_eq!(minimum_distance(points), 10);
    }
}
