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

    pub fn map<U>(&self, f: impl FnMut(&T) -> U) -> Grid<U> {
        Grid {
            size: self.size,
            data: self.data.iter().map(f).collect(),
        }
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
    T: Copy,
{
    fn from(value: &Vec<Vec<T>>) -> Self {
        let size_x = value.len();
        let size_y = value[0].len();
        assert!(value.iter().map(|row| row.len()).all(|len| len == size_y));

        let mut data = Vec::new();
        for v in value {
            data.extend_from_slice(v);
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
        let start = Coordinate::default();
        let mut end = self;
        end.x.step_back(start.x);
        end.y.step_back(start.y);
        RangeIter {
            start,
            end,
            min: start,
            max: self,
            complete: start.x >= self.x || start.y >= self.y,
        }
    }
}

struct RangeIter<T> {
    start: Coordinate<T>,
    end: Coordinate<T>,
    min: Coordinate<T>,
    max: Coordinate<T>,
    complete: bool,
}

impl<T> RangeIter<T>
where
    T: Step,
{
    fn next_internal(
        &mut self,
        f_curr_value: impl FnOnce(&Self) -> Coordinate<T>,
        f_next: impl FnOnce(&mut Self),
    ) -> Option<Coordinate<T>> {
        if self.complete {
            return None;
        }
        if self.start.x > self.end.x || self.start.x == self.end.x && self.start.y > self.end.y {
            self.complete = true;
            return None;
        }
        if self.start.x == self.end.x && self.start.y == self.end.y {
            self.complete = true;
            return Some(f_curr_value(self));
        }

        let ret = f_curr_value(self);
        f_next(self);
        Some(ret)
    }
}

impl<T> Iterator for RangeIter<T>
where
    T: Step,
{
    type Item = Coordinate<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_internal(
            |s| s.start,
            |s| {
                if !s.start.y.step(s.max.y) {
                    if !s.start.x.step(s.max.x) {
                        unreachable!("end value is larger then max value")
                    }
                    s.start.y = s.min.y;
                }
            },
        )
    }
}

impl<T> DoubleEndedIterator for RangeIter<T>
where
    T: Step,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_internal(
            |s| s.end,
            |s| {
                if !s.end.y.step_back(s.min.y) {
                    if !s.end.x.step_back(s.min.x) {
                        unreachable!("start value is smaller then min value")
                    }
                    s.end.y = s.max.y;
                    s.end.y.step_back(s.min.y);
                }
            },
        )
    }
}

pub trait Step: Default + Copy + PartialOrd {
    fn step(&mut self, max: Self) -> bool;
    fn step_back(&mut self, min: Self) -> bool;
}

impl Step for usize {
    fn step(&mut self, max: Self) -> bool {
        if *self + 1 >= max {
            return false;
        }
        *self += 1;
        true
    }

    fn step_back(&mut self, min: Self) -> bool {
        if *self <= min {
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
