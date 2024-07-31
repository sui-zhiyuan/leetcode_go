pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
    let mut x_axis = points.iter().map(|p| p[0]).collect::<Vec<i32>>();
    x_axis.sort();
    let n = x_axis.len();

    let mut cover = vec![i32::MAX ; n +1];
    cover[0] = 0;

    for i in 1..=n {
        let p = binary_search(0, i-1, |j| x_axis[j] + w >= x_axis[i-1]);
        cover[i] = cover[p] + 1;
    }
    cover[n]
}

fn binary_search(mut l: usize , mut r:usize , mut f: impl FnMut(usize) -> bool) -> usize {
    while l+1 < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m;
        }
    }
    if f(l) {
        l
    } else {
        r
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test1(){
        let points = [[2,1],[1,0],[1,4],[1,8],[3,5],[4,6]];
        let points = points.iter().map(|p| p.to_vec()).collect();
        let w = 1;
        assert_eq!(min_rectangles_to_cover_points(points, w), 2);
    }
}