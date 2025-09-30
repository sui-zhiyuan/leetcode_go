use std::fmt::{Debug, Formatter};
use std::ops::{Add, Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    size: Coordinate,
}

impl<T> Grid<T> {
    pub fn try_get(&self, c: impl Into<Coordinate>) -> Option<&T> {
        self.data.get(self.get_index(c.into())?)
    }

    pub fn try_get_mut(&mut self, c: impl Into<Coordinate>) -> Option<&mut T> {
        let i = self.get_index(c.into())?;
        self.data.get_mut(i)
    }

    pub fn get(&self, c: impl Into<Coordinate>) -> &T {
        let c: Coordinate = c.into();
        match self.try_get(c) {
            Some(t) => t,
            None => panic!("index {c:?} out of range {:?}", self.size),
        }
    }

    pub fn get_mut(&mut self, c: impl Into<Coordinate>) -> &mut T {
        let c: Coordinate = c.into();
        let size = self.size;
        match self.try_get_mut(c) {
            Some(t) => t,
            None => panic!("index {c:?} out of range {size:?}"),
        }
    }

    fn get_index(&self, c: Coordinate) -> Option<usize> {
        if c.inside(self.size) {
            Some(c.x * self.size.y + c.y)
        } else {
            None
        }
    }

    pub fn size(&self) -> Coordinate {
        self.size
    }

    pub fn map<U>(&self, mut f: impl FnMut(&T) -> U) -> Grid<U> {
        let mut ret = Grid {
            size: self.size,
            data: Vec::with_capacity(self.size.x * self.size.y),
        };
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                ret.data.push(f(self.get((x, y))));
            }
        }
        ret
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(size: impl Into<Coordinate>, value: T) -> Self {
        let size: Coordinate = size.into();
        Self {
            data: vec![value; size.x * size.y],
            size,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let size_x = value.len();
        let size_y = value[0].len();
        assert!(value.iter().map(|row| row.len()).all(|len| len == size_y));

        let mut data = Vec::new();
        for v in value {
            data.extend(v);
        }

        Grid {
            data,
            size: (size_x, size_y).into(),
        }
    }
}

impl<T> From<&Vec<Vec<T>>> for Grid<T>
where
    T: Clone,
{
    fn from(value: &Vec<Vec<T>>) -> Self {
        let size_x = value.len();
        let size_y = value[0].len();
        assert!(value.iter().map(|row| row.len()).all(|len| len == size_y));

        let mut data = Vec::new();
        for v in value {
            data.extend(v.clone());
        }

        Grid {
            data,
            size: (size_x, size_y).into(),
        }
    }
}

impl<T> From<Grid<T>> for Vec<Vec<T>> {
    fn from(val: Grid<T>) -> Self {
        let mut grid = Vec::with_capacity(val.size.x);
        let mut row = Vec::with_capacity(val.size.y);
        for (i, v) in val.data.into_iter().enumerate() {
            row.push(v);
            if i % val.size.y == val.size.y - 1 {
                grid.push(row);
                row = Vec::with_capacity(val.size.x);
            }
        }
        grid
    }
}

impl<T, K> Index<K> for Grid<T>
where
    K: Into<Coordinate>,
{
    type Output = T;

    fn index(&self, index: K) -> &Self::Output {
        self.get(index)
    }
}

impl<T, K> IndexMut<K> for Grid<T>
where
    K: Into<Coordinate>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.size.iter().map(|c| &self[c])
    }

    // todo not allow mut
    // pub fn iter_mut<>(&mut self) -> impl Iterator<Item = &mut T> {
    //     self.size.iter().map(|c| &mut self[c])
    // }

    pub fn iter_coor(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.size.iter().map(|c| (c, &self[c]))
    }

    // pub fn iter_coor_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> {
    //     self.size.iter().map(|c| (c, &mut self[c]))
    // }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Coordinate<T = usize> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Coordinate<T>
where
    T: Ord,
{
    pub fn inside(&self, c: Self) -> bool {
        self.x < c.x && self.y < c.y
    }
}

impl<T> Debug for Coordinate<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y)
    }
}

impl<T> From<(T, T)> for Coordinate<T> {
    fn from((x, y): (T, T)) -> Self {
        Coordinate { x, y }
    }
}

impl<T> Add for Coordinate<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Coordinate<usize> {
    pub fn checked_add(self, rhs: Coordinate<usize>) -> Option<Coordinate<usize>> {
        match (self.x.checked_add(rhs.x), self.y.checked_add(rhs.y)) {
            (Some(x), Some(y)) => Some(Coordinate { x, y }),
            _ => None,
        }
    }

    pub fn checked_add_signed(self, rhs: Coordinate<isize>) -> Option<Coordinate<usize>> {
        match (
            self.x.checked_add_signed(rhs.x),
            self.y.checked_add_signed(rhs.y),
        ) {
            (Some(x), Some(y)) => Some(Coordinate { x, y }),
            _ => None,
        }
    }
}

// refactor after `std::Step` stabilized
impl<T> Coordinate<T>
where
    T: Step + Default + PartialOrd + Copy,
{
    pub fn iter(self) -> impl DoubleEndedIterator<Item = Coordinate<T>> {
        let mut end_x = self.x;
        let mut end_y = self.y;
        end_x.step_back();
        end_y.step_back();
        RangeIter {

            start: Coordinate::default(),
            end: Coordinate {
                x: self.x.st(),
                y: T::default(),
            },
            max: self,
        }
    }
}

struct RangeIter<T> {
    start: Coordinate<T>,
    end: Coordinate<T>,
    max: Coordinate<T>,
}

impl<T> Iterator for RangeIter<T>
where
    T: Step + Default + PartialOrd + Copy,
{
    type Item = Coordinate<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.x > self.end.y || self.start.x == self.end.x && self.start.x >= self.end.y {
            return None;
        }
        let ret = self.start;
        if !self.start.y.step(self.max.y) {
            if !self.start.x.step(self.max.x) {
                return None;
            }
            self.start.y = T::default();
        }
        Some(ret)
    }
}

impl<T> DoubleEndedIterator for RangeIter<T>
where
    T: Step + Default + PartialOrd + Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.end.y.step_back() {
            if !self.end.x.step_back() {
                return None;
            }
            self.end.y = self.max.y;
        }

        if self.start.x > self.end.y || self.start.x == self.end.x && self.start.x > self.end.y {
            return None;
        }

        Some(self.end)
    }
}

pub trait Step {
    fn step(&mut self, max: Self) -> bool;
    fn step_back(&mut self) -> bool;
}

impl Step for usize {
    fn step(&mut self, max: Self) -> bool {
        if *self + 1 >= max {
            return false;
        }
        *self += 1;
        true
    }

    fn step_back(&mut self) -> bool {
        if *self == 0 {
            return false;
        }
        *self -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let coor = Coordinate {
            x: 2usize,
            y: 3usize,
        };

        let mut expect = Vec::new();

        for x in 0..coor.x {
            for y in 0..coor.y {
                expect.push(Coordinate { x, y });
            }
        }

        let expect_rev = expect.iter().copied().rev().collect::<Vec<_>>();

        assert_eq!(expect, coor.iter().collect::<Vec<Coordinate>>());
        assert_eq!(expect_rev, coor.iter().rev().collect::<Vec<Coordinate>>());
    }
}
