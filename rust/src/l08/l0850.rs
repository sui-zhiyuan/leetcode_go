use std::{
    cmp,
    collections::{BTreeSet, HashMap},
};

use crate::common::SegmentTree;

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        const DIV_MOD: u64 = 1_000_000_007;
        let rectangles = rectangles
            .iter()
            .filter(|v| v[0] < v[2] && v[1] < v[3])
            .map(|v| Rectangle::from(&v[..]))
            .collect::<Vec<_>>();

        let x_list = rectangles
            .iter()
            .flat_map(|r| vec![r.x1, r.x2])
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let y_map = rectangles
            .iter()
            .flat_map(|r| vec![(r.y1, r), (r.y2, r)])
            .fold(HashMap::<_, Vec<&Rectangle>>::new(), |mut m, (y, r)| {
                m.entry(y).or_default().push(r);
                m
            });

        let y_list = y_map
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let x_map = x_list
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<HashMap<_, _>>();

        let lines = (1..x_list.len())
            .map(|i| Line {
                length: x_list[i] - x_list[i - 1],
                count: 0,
            })
            .collect::<Vec<_>>();

        // dbg!(&x_list, &y_list, &x_map , &y_map, &lines);

        let mut segment_tree = SegmentTree::new(lines, merge_line);
        let mut prev_y = 0;
        let mut area = 0;
        for y in y_list {
            let dx = u64::from(segment_tree.range(0, segment_tree.len()).length());
            let dy = u64::from(y - prev_y);
            prev_y = y;
            area = (area + dx * dy) % DIV_MOD;

            for &r in y_map.get(&y).unwrap() {
                let x1 = x_map[&r.x1];
                let x2 = x_map[&r.x2];
                let change = if y == r.y1 {
                    |v: &mut Line| v.count += 1
                } else if y == r.y2 {
                    |v: &mut Line| v.count -= 1
                } else {
                    unreachable!()
                };

                for k in x1..x2 {
                    segment_tree.change_value(k, change);
                }
            }

            // dbg!(y, area, &segment_tree);
        }

        area.try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct Rectangle {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl From<&[i32]> for Rectangle {
    fn from(data: &[i32]) -> Self {
        assert!(data.len() == 4, "data.len():{} != 4", data.len());
        assert!(
            data[0] < data[2],
            "data[0]:{} >= data[2]:{}",
            data[0],
            data[2]
        );
        assert!(
            data[1] < data[3],
            "data[1]:{} >= data[3]:{}",
            data[1],
            data[3]
        );
        Rectangle {
            x1: data[0].try_into().unwrap(),
            y1: data[1].try_into().unwrap(),
            x2: data[2].try_into().unwrap(),
            y2: data[3].try_into().unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Line {
    length: u32,
    count: u32,
}

impl Line {
    pub fn length(&self) -> u32 {
        if self.count == 0 {
            return 0;
        }
        self.length
    }
}

fn merge_line(l1: Line, l2: Line) -> Line {
    Line {
        length: l1.length() + l2.length(),
        count: cmp::max(l1.count, l2.count),
    }
}

pub struct Solution();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let input = vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]];
        let output = 6;
        assert_eq!(Solution::rectangle_area(input), output);

        let input = vec![vec![0, 0, 1000000000, 1000000000]];
        let output = 49;
        assert_eq!(Solution::rectangle_area(input), output);
    }
}
