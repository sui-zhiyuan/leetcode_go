use std::{
    iter::IntoIterator,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

#[derive(Debug, Clone)]
pub struct ExtendVec<T> {
    data: Vec<T>,
    left: isize,
}

impl<T> ExtendVec<T>
where
    T: Clone,
{
    pub fn new(left: isize, right: isize, value: T) -> Self {
        ExtendVec {
            data: vec![value; (right - left) as usize],
            left,
        }
    }

    pub fn get(&self, index: isize) -> T {
        self.data[(index - self.left) as usize].clone()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

impl<T> Index<isize> for ExtendVec<T> {
    type Output = T;

    fn index(&self, index: isize) -> &T {
        &self.data[(index - self.left) as usize]
    }
}

impl<T> IndexMut<isize> for ExtendVec<T> {
    fn index_mut(&mut self, index: isize) -> &mut T {
        &mut self.data[(index - self.left) as usize]
    }
}

impl<T> IntoIterator for ExtendVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ExtendVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
