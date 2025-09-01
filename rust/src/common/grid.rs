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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    fn inside(&self, c: Coordinate) -> bool {
        self.x < c.x && self.y < c.y
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Coordinate { x, y }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
