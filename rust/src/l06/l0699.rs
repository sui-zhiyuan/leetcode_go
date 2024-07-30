pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pos: Vec<Segment> = vec![];
    let mut result = vec![];
    for s in positions {
        let l = s[0];
        let h = s[1];
        let r = l + h;

        let mut start = 0;
        let mut end = 0;
        let mut max_h = 0;
        for (i, s) in pos.iter().enumerate() {
            if s.r <= l {
                start = i+1;
            }
            if s.l < r {
                end = i+1;
            }

            if s.r > l && s.l < r {
                max_h = max_h.max(s.h)
            }
        }
        let new_h = max_h + h;
        let last_h = result.last().copied().unwrap_or(0);
        result.push(last_h.max(new_h));

        let mut new: Vec<Segment> = vec![];
        if max_h > 0 && pos[start].l < l {
            new.push(Segment {
                l: pos[start].l,
                r: l,
                h: pos[start].h,
            });
        }
        new.push(Segment {
            l,
            r,
            h: max_h + h,
        });
        if max_h > 0 && end-1 < pos.len() && pos[end-1].r > r {
            new.push(Segment {
                l: r,
                r: pos[end-1].r,
                h: pos[end-1].h,
            });
        }

        pos.splice(start..end, new);
    }

    result
}

struct Segment {
    l: i32,
    r: i32,
    h: i32,
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let positions = [[1,2],[2,3],[6,1]];
        let positions: Vec<Vec<i32>> = positions.iter().map(|x| x.to_vec()).collect();
        let result = falling_squares(positions);
        assert_eq!(result, vec![2, 5, 5]);
    }
}