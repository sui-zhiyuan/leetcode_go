
use crate::common::DisjointSet;

pub fn can_reach_corner(x: i32, y: i32, circles: Vec<Vec<i32>>) -> bool {
    let n=circles.len();
    let circles = circles.iter().map(|c| Circle{x:c[0], y:c[1], r:c[2]}).collect::<Vec<_>>();

    let touches = circles.iter().map(|c| {
        Touch{
            top: c.y + c.r >= y,
            bottom: c.y - c.r <= 0,
            left: c.x - c.r <= 0,
            right: c.x + c.r >= x,
        }
    }).collect::<Vec<_>>();

    let mut dsj = DisjointSet::new(&touches , Touch::or);

    for i in 0..n{
        for j in i+1..n{
            if circles[i].touch(&circles[j]){
                dsj.union(i, j);
            }
        }
    }

    for i in 0..n {
        let v = dsj.value(i);
        if v.top && v.bottom || v.left && v.right || v.bottom && v.left || v.top && v.right {
            return false;
        }
    }

    true
}

struct Circle{
    x:i32,
    y:i32,
    r:i32,
}

impl Circle{
    fn touch(&self, other:&Circle)->bool{
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let d = (dx*dx + dy*dy) as f64;
        let r = (self.r + other.r) as f64;
        d <= r*r
    }
}

#[derive(Clone, Copy)]
struct Touch{
    top:bool,
    bottom:bool,
    left:bool,
    right:bool,
}

impl Touch{
    fn or(self, other:Touch)->Touch{
        Touch{
            top:self.top || other.top,
            bottom:self.bottom || other.bottom,
            left:self.left || other.left,
            right:self.right || other.right,
        }
    }
}