#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//TODO: better documentation
//TODO: better names
//TODO: change to make own struct with methods, separate from Self
//TODO: run one by one, and then make method with self for final vec

mod array2d;
pub use array2d::*;

use bitvec::prelude::BitVec;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct WFCGenerator<T: 'static> {
    ///Width of the final map to give out
    width: usize,
    ///Height of the final map to give out
    height: usize,
    ///All the possible variants
    variants: &'static [T],
    ///The number of variants
    variant_nos: usize,
    ///The possibilities of any given state
    neighbour_possibilities: HashMap<usize, BitVec>,
    ///The map we're currently working on
    in_progress_map: Array2D<BitVec>,
    ///The type we're using for the states
    ty: PhantomData<T>,
    ///The next tile to work out
    next_to_generate: (usize, usize),
}

impl<T: Clone + 'static + WFCState> WFCGenerator<T> {
    ///Create a new [`BitVec`] of a given size with a given default value, like `vec![]`
    fn filled_bitvec(size: usize, default_value: bool) -> BitVec {
        let mut bv = BitVec::new();
        (0..size).for_each(|_| bv.push(default_value));
        bv
    }
    ///Whether or not a given Array2D in the range, contains exclusively bitvecs with 1 bit set to true.
    fn finished(
        a2d: &Array2D<BitVec>,
        w: impl Iterator<Item = usize>,
        h: impl Iterator<Item = usize> + Clone,
    ) -> bool {
        for x in w {
            for y in h.clone() {
                if a2d[(x, y)].count_ones() > 1 {
                    return false;
                }
            }
        }
        true
    }
    ///Extracts an index from a bitvec - returns None if no index is found, or there are more than one indices
    fn extract_index(from: &BitVec) -> Option<usize> {
        if from.count_ones() == 1 {
            from.iter()
                .enumerate()
                .find_map(|(i, b)| if *b { Some(i) } else { None })
        } else {
            None
        }
    }

    #[allow(clippy::must_use_candidate)]
    pub fn new(width: usize, height: usize) -> Self {
        let variants = T::get_variants();
        let variant_nos = variants.len();
        Self {
            width,
            height,
            variants,
            variant_nos,
            neighbour_possibilities: T::possible_neighbours()
                .into_iter()
                .map(|(t, v)| {
                    let mut bv = Self::filled_bitvec(variant_nos, false);
                    for neighbour in v {
                        bv.set(neighbour.to_usize(), true);
                    }
                    (t.to_usize(), bv)
                })
                .collect(),
            in_progress_map: Array2D::filled(width, height, Self::filled_bitvec(variant_nos, true)),
            next_to_generate: (0, 0),
            ty: PhantomData,
        }
    }

    ///Steps through generation, returning whether or not it is finished
    #[must_use]
    pub fn step(&mut self) -> bool {
        let (x, y) = self.next_to_generate;

        if x == self.width - 1 && y == self.height - 1 {
            return true;
        }
        if x < self.width {
            self.next_to_generate.0 += 1;
        } else {
            self.next_to_generate.0 = 0;
            self.next_to_generate.1 += 1;
        }

        for (dx, dy) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            if dx == 0 && dy == 0 {
                continue;
            }

            if let Some((new_x, new_y)) = self.get_with_delta((x, y), dx, dy) {
                if let Some(i) = Self::extract_index(&self.in_progress_map[(new_x, new_y)]) {
                    self.in_progress_map[(x, y)] &= self.neighbour_possibilities[&i].clone();
                }
            }
        }

        let mut rng = thread_rng();

        let mut possibilities_matrix = self.in_progress_map[(x, y)]
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i) } else { None })
            .collect_vec();

        possibilities_matrix.append(&mut possibilities_matrix.clone());

        let mut biased_possibilities_matrix = vec![];

        for i in possibilities_matrix {
            for _ in 0..self.variants[i].bias() {
                biased_possibilities_matrix.push(i);
            }
        }

        for dx in -2..=2 {
            for dy in -2..=2 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if let Some(pos) = self.get_with_delta((x, y), dx, dy) {
                    //neighbour bias
                    if let Some(i) = Self::extract_index(&self.in_progress_map[pos]) {
                        for _ in 0..(self.variants[i].bias()) {
                            biased_possibilities_matrix.push(i);
                        }
                    }
                }
            }
        }

        self.in_progress_map[(x, y)].fill(false);
        self.in_progress_map[(x, y)].set(
            biased_possibilities_matrix[rng.gen_range(0..biased_possibilities_matrix.len())],
            true,
        );

        false
    }

    #[allow(clippy::must_use_candidate)]
    pub const fn get_with_delta(
        &self,
        (x, y): (usize, usize),
        dx: isize,
        dy: isize,
    ) -> Option<(usize, usize)> {
        if let Some(x) = x.checked_add_signed(dx) {
            if x < self.width {
                if let Some(y) = y.checked_add_signed(dy) {
                    if y < self.height {
                        return Some((x, y));
                    }
                }
            }
        }

        None
    }

    ///Finishes - panics if we can't find indices and aren't properly finished yet
    pub fn finish(self) -> Vec<T> {
        let mut nv = Vec::with_capacity(self.width * self.height);
        for x in 1..=self.width {
            for y in 1..=self.height {
                let index = Self::extract_index(&self.in_progress_map[(x, y)]).unwrap();
                nv.push(self.variants[index].clone());
            }
        }
        nv
    }

    ///Gets current without panicking
    pub fn get_current(&self) -> Vec<Option<T>> {
        let mut nv = Vec::with_capacity(self.width * self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(i) = Self::extract_index(&self.in_progress_map[(x, y)]) {
                    nv.push(Some(self.variants[i].clone()));
                } else {
                    nv.push(None);
                }
            }
        }
        nv
    }
}

pub trait WFCState: Clone + 'static {
    ///All the possible variants for a given Self
    ///
    ///Is ideally deterministic
    fn get_variants() -> &'static [Self];

    ///Deterministic fashion to convert variants into usizes - NB: all must be unique
    fn to_usize(self) -> usize;

    ///Bias from 1 to 5
    fn bias(&self) -> usize;

    ///Returns all the possible neighbours for each Self
    fn possible_neighbours() -> HashMap<Self, Vec<Self>>;
}
