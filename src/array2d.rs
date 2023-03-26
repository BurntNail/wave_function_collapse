use std::ops::{Index, IndexMut};
use std::vec::IntoIter;

#[derive(Default, Debug, Clone)]
pub struct Array2D<T> {
    backing: Vec<T>,
    width: usize,
}

impl<T> Array2D<T> {
    pub fn add_element(&mut self, el: T) {
        self.backing.push(el);
    }
    pub fn remove(&mut self, index: (usize, usize)) -> T {
        return self
            .backing
            .remove(Self::coords_to_index(self.width, index));
    }
    pub fn to_inner(self) -> Vec<T> {
        self.backing
    }
    pub fn contains(&self, cnd: impl Fn(&T) -> bool) -> bool {
        self.backing.iter().any(cnd)
    }

    pub fn index_to_coords(width: usize, index: usize) -> (usize, usize) {
        (index / width, index % width)
    }
    pub fn coords_to_index(width: usize, (x, y): (usize, usize)) -> usize {
        y * width + x
    }

    pub fn len(&self) -> usize {
        self.backing.len()
    }
    pub fn size(&self) -> (usize, usize) {
        (
            self.width,
            (self.len() as f32 / self.width as f32).ceil() as usize,
        )
    }
}

impl<T: Clone> Array2D<T> {
    pub fn filled(width: usize, height: usize, el: T) -> Self {
        Self {
            backing: vec![el; width * height],
            width,
        }
    }
}

impl<T> IntoIterator for Array2D<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.backing.into_iter()
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.backing[Self::coords_to_index(self.width, index)]
    }
}
impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.backing[Self::coords_to_index(self.width, index)]
    }
}
