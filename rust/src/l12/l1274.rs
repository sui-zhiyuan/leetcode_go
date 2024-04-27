// cspell:ignore coor

pub fn count_ships(sea: &Sea, top_right: Vec<i32>, bottom_left: Vec<i32>) -> i32 {
    let top_right = Coor::from(top_right).add1();
    let bottom_left = Coor::from(bottom_left);

    count_ships_inner(sea, bottom_left, top_right)
}

// fn count_ships_inner(sea: &mut Sea, bottom_left: Coor, top_right: Coor) -> i32 {
//     let result = count_ships_inner1(sea, bottom_left, top_right);
//     dbg!((bottom_left, top_right, result));
//     result
// }

fn count_ships_inner(sea: &Sea, bottom_left: Coor, top_right: Coor) -> i32 {
    if top_right.sub1() == bottom_left {
        return if sea.has_ships(bottom_left.into(), bottom_left.into()) {
            1
        } else {
            0
        };
    }

    if !sea.has_ships(top_right.sub1().into(), bottom_left.into()) {
        return 0;
    }

    if bottom_left.y + 1 == top_right.y {
        let mid_x = (bottom_left.x + top_right.x) / 2;
        let mut count = 0;
        count += count_ships_inner(sea, bottom_left, Coor::new(mid_x, top_right.y));
        count += count_ships_inner(sea, Coor::new(mid_x, bottom_left.y), top_right);
        return count;
    }

    let mid_y = (bottom_left.y + top_right.y) / 2;
    let mut count = 0;
    count += count_ships_inner(sea, bottom_left, Coor::new(top_right.x, mid_y));
    count += count_ships_inner(sea, Coor::new(bottom_left.x, mid_y), top_right);
    count
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Coor {
    x: i32,
    y: i32,
}

impl Coor {
    fn new(x: i32, y: i32) -> Coor {
        Coor { x, y }
    }
    fn add1(&self) -> Coor {
        Coor {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn sub1(&self) -> Coor {
        Coor {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
}

impl From<Vec<i32>> for Coor {
    fn from(value: Vec<i32>) -> Self {
        Coor {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<Coor> for Vec<i32> {
    fn from(val: Coor) -> Self {
        vec![val.x, val.y]
    }
}

// debug and test
use std::{cell::RefCell, fmt::Debug};

impl Debug for Coor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

pub struct Sea {
    ships: Vec<Vec<i32>>,
    access: RefCell<usize>,
}
impl Sea {
    pub fn new(value: Vec<Vec<i32>>) -> Sea {
        Sea {
            ships: value,
            access: RefCell::new(0),
        }
    }
    pub fn has_ships(&self, top_right: Vec<i32>, bottom_left: Vec<i32>) -> bool {
        let mut access = self.access.borrow_mut();
        *access += 1;
        if *access >= 400 {
            panic!("Access Limit Exceeded");
        }

        for ship in self.ships.iter() {
            if ship[0] >= bottom_left[0]
                && ship[0] <= top_right[0]
                && ship[1] >= bottom_left[1]
                && ship[1] <= top_right[1]
            {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::parse_grid;

    #[test]
    fn test1() {
        let sea = parse_grid("[[1,1],[2,2],[3,3],[5,5]]").unwrap();
        let sea = &mut Sea::new(sea);
        assert_eq!(3, count_ships(sea, vec![4, 4], vec![0, 0]));
    }
}
