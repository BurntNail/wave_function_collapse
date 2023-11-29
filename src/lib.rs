#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//TODO: better documentation
//TODO: better names

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
    fn extract_index(from: &BitVec, print: bool) -> Option<usize> {
        if from.count_ones() == 1 {
            from.iter()
                .enumerate()
                .find_map(|(i, b)| if *b { Some(i) } else { None })
        } else {
            if print {
                println!("Found {} ones", from.count_ones());
            }
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
    pub fn step_moar_random(&mut self) -> bool {
        if Self::finished(&self.in_progress_map, 0..self.width, 0..self.height) {
            return true;
        }

        let deltas = (-2..=2).permutations(2);

        for x in 0..self.width {
            for y in 0..self.height {
                for (dx, dy) in deltas.clone().into_iter().map(|mut x| (x.remove(0), x.remove(0))) {
                   if let Some((new_x, new_y)) = self.get_with_delta((x, y), dx, dy) {
                        if let Some(i) = Self::extract_index(&self.in_progress_map[(new_x, new_y)], false)
                        {
                            self.in_progress_map[(x, y)] &=
                                self.neighbour_possibilities[&i].clone();
                        }
                    }
                }
            }
        }


        let mut min_possibilities = usize::MAX;
        let mut min_list = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let possibilities = self.in_progress_map[(x, y)].count_ones();

                if possibilities <= 1 {
                    continue;
                }

                if possibilities < min_possibilities {
                    min_possibilities = possibilities;
                    min_list.clear();
                }

                if possibilities == min_possibilities {
                    min_list.push((x, y));
                }
            }
        }

        if min_list.len() == 0 {
            return true;
        }

        let mut rng = thread_rng();
        let (x, y) = min_list.remove(rng.gen_range(0..min_list.len()));

        let mut possibilities_matrix = self.in_progress_map[(x, y)]
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i) } else { None })
            .collect_vec();

        for dx in -2..=2 {
            for dy in -2..=2 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if let Some(pos) = self.get_with_delta((x, y), dx, dy) {
                    //neighbour bias
                    if let Some(neighbour_index) = Self::extract_index(&self.in_progress_map[pos], false) {
                        for possible_option in self.in_progress_map[(x, y)]
                            .iter()
                            .enumerate()
                            .filter_map(|(i, b)| if *b { Some(&self.variants[i]) } else { None })
                        {
                            for _ in 0..(self.variants[neighbour_index].bias(possible_option)) {
                                possibilities_matrix.push(neighbour_index);
                            }
                        }
                    }
                }
            }
        }

        self.in_progress_map[(x, y)].fill(false);
        self.in_progress_map[(x, y)].set(
            if possibilities_matrix.is_empty() {
                self.variants[rng.gen_range(0..self.variant_nos)]
                    .clone()
                    .to_usize()
            } else {
                possibilities_matrix[rng.gen_range(0..possibilities_matrix.len())]
            },
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
    pub fn finish(mut self) -> Vec<T> {
        let mut nv = Vec::with_capacity(self.width * self.height);
        let mut rng = thread_rng();

        for x in 0..self.width {
            for y in 0..self.height {
                let mut current: Vec<usize> = self.in_progress_map[(x, y)].iter().enumerate().filter_map(|(i, bool)| if *bool {Some(i)} else {None}).collect();
                if current.len() > 1 {
                    let picked = current.remove(rng.gen_range(0..current.len()));
                    self.in_progress_map[(x, y)].clear();
                    self.in_progress_map[(x, y)].set(picked, true);
                }


                let variant = match Self::extract_index(&self.in_progress_map[(x, y)], true) {
                    Some(x) => self.variants[x].clone(),
                    None => T::none_option(),
                };
                nv.push(variant);
            }
        }
        nv
    }

    ///Gets current without panicking
    pub fn get_current(&self) -> Vec<Option<T>> {
        let mut nv = Vec::with_capacity(self.width * self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(i) = Self::extract_index(&self.in_progress_map[(x, y)], false) {
                    nv.push(Some(self.variants[i].clone()));
                } else {
                    nv.push(None);
                }
            }
        }
        nv
    }

    pub fn get_no_done (&self) -> usize {
        let mut wk = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.in_progress_map[(x, y)].count_ones() <= 1 {
                    wk += 1;
                }
            }
        }

        wk
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
    fn bias(&self, o: &Self) -> usize;

    ///Returns all the possible neighbours for each Self
    fn possible_neighbours() -> HashMap<Self, Vec<Self>>;

    fn none_option() -> Self;
}
