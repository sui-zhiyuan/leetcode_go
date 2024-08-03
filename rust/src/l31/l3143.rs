use std::collections::HashSet;

pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
    let points = points.iter().zip(s.chars()).map(|(p, l)| Point {
        x: p[0],
        y: p[1],
        l,
    });

    let mut points = points
        .map(|p| (p.x.abs().max(p.y.abs()), p.l))
        .collect::<Vec<_>>();

    points.sort_by_key(|(d, _)| *d);

    let mut find = HashSet::new();
    let mut curr = 0;

    'outer: while curr < points.len() {
        let p_next= points[curr].0;
        let mut next = curr;
        while next < points.len() && points[next].0 == p_next {
            if find.contains(&points[next].1) {
                break 'outer;
            }
            find.insert(points[next].1);
            next += 1;
        }
        curr = next;
    }


    curr as i32
}

struct Point {
    x: i32,
    y: i32,
    l: char,
}
