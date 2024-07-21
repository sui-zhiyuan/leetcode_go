use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Dim2Array<T> {
    data: Vec<T>,
    size_x: usize,
    size_y: usize,
}

impl<T> Dim2Array<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x<self.size_x && y<self.size_y, "out of range: x={}, y={}", x, y);
        &self.data[y * self.size_x + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        assert!(x<self.size_x && y<self.size_y, "out of range: x={}, y={}", x, y);
        &mut self.data[y * self.size_x + x]
    }
}

impl<T> Dim2Array<T>
where
    T: Clone,
{
    pub fn new(size_x: usize, size_y: usize, value: T) -> Self {
        Self {
            data: vec![value; size_x * size_y],
            size_x,
            size_y,
        }
    }
}

impl<T> Index<(usize, usize)> for Dim2Array<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<T> IndexMut<(usize, usize)> for Dim2Array<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
    }
}