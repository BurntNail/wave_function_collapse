use std::ops::{Index, IndexMut};
use std::vec::IntoIter;

///Structure to act as a 2D Array, generic over T.
///
///The array has infinite height, and we just keep adding rows as elements are added.
#[derive(Default, Debug, Clone)]
pub struct Array2D<T> {
    backing: Vec<T>,
    width: usize,
}

///Given a width, turn an index into an (x, y) position
pub fn index_to_coords(width: usize, index: usize) -> (usize, usize) {
    (index / width, index % width)
}
///Given a width and an (x, y) position, find the index
pub fn coords_to_index(width: usize, (x, y): (usize, usize)) -> usize {
    y * width + x
}

impl<T> Array2D<T> {
    ///Add an element to the next available position in the grid
    pub fn add_element(&mut self, el: T) {
        self.backing.push(el);
    }

    ///Remove a given index - similar to [`Vec::remove`], this will panic with an invalid index.
    ///
    ///NB: This also moves all elements after the chosen index left (and up) and so can change rows
    pub fn remove(&mut self, index: (usize, usize)) -> T {
        return self.backing.remove(coords_to_index(self.width, index));
    }
    ///Get the inner vector
    pub fn to_inner(self) -> Vec<T> {
        self.backing
    }
    ///Check whether any items fufill a condition given by a closure
    pub fn contains(&self, cnd: impl Fn(&T) -> bool) -> bool {
        self.backing.iter().any(cnd)
    }

    pub fn len(&self) -> usize {
        self.backing.len()
    }
    pub fn is_empty(&self) -> bool {
        self.backing.is_empty()
    }

    ///Get the (columns, rows)
    pub fn size(&self) -> (usize, usize) {
        (
            self.width,
            (self.len() as f32 / self.width as f32).ceil() as usize,
        )
    }
}

impl<T: Clone> Array2D<T> {
    ///Create an array with given width and height filled with clones of `el`
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
        &self.backing[coords_to_index(self.width, index)]
    }
}
impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.backing[coords_to_index(self.width, index)]
    }
}
