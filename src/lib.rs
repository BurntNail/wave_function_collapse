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

    ///Generates a new board, in english order (left to right, top to bottom)
    #[allow(clippy::too_many_lines)]
    fn generate(width: usize, height: usize) -> Vec<Self> {
        fn filled_bitvec(size: usize, default_value: bool) -> BitVec {
            let mut bv = BitVec::new();
            (0..size).for_each(|_| bv.push(default_value));
            bv
        }

        fn finished(a2d: &Array2D<BitVec>, w: usize, h: usize) -> bool {
            for x in 1..=w {
                for y in 1..=h {
                    if a2d[(x, y)].count_ones() > 1 {
                        return false;
                    }
                }
            }
            true
        }

        fn extract_index(from: &BitVec) -> Option<usize> {
            if from.count_ones() == 1 {
                from.iter()
                    .enumerate()
                    .find_map(|(i, b)| if *b { Some(i) } else { None })
            } else {
                None
            }
        }
        let (tw, th) = (width + 2, height + 2);
        let variants = Self::get_variants();

        let variant_nos = variants.len();
        let neighbour_possibilities: HashMap<usize, BitVec> = Self::possible_neighbours()
            .into_iter()
            .map(|(t, v)| {
                let mut bv = filled_bitvec(variant_nos, false);
                for neighbour in v {
                    bv.set(neighbour.to_usize(), true);
                }
                (t.to_usize(), bv)
            })
            .collect();

        let mut a2d: Array2D<BitVec> = Array2D::filled(tw, th, filled_bitvec(variant_nos, true)); // account for extra top/btm columns

        while !finished(&a2d, width, height) {
            let mut least_possibilities = None;

            for x in 1..(tw - 1) {
                for y in 1..(th - 1) {
                    if a2d[(x, y)].count_ones() == 1 {
                        continue;
                    }

                    let mut here = filled_bitvec(8, true);
                    for (dx, dy) in [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ] {
                        let neighbour = &a2d[((x as i32 + dx) as usize, (y as i32 + dy) as usize)];

                        for i in
                            neighbour.iter().enumerate().filter_map(
                                |(i, b)| {
                                    if *b {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                },
                            )
                        {
                            here |= neighbour_possibilities[&i].clone();
                        }
                    }

                    a2d[(x, y)] &= here;

                    let ones_here = a2d[(x, y)].count_ones();
                    if ones_here > 1
                        && least_possibilities.map_or(true, |lp| a2d[lp].count_ones() > ones_here)
                    {
                        least_possibilities = Some((x, y));
                    }
                }
            }

            if let Some(lp) = least_possibilities {
                let mut possibilities_matrix = a2d[lp]
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, b)| if b { Some(i) } else { None })
                    .collect_vec();

                possibilities_matrix.append(&mut possibilities_matrix.clone());

                let mut biased_possibilities_matrix = vec![];

                for i in possibilities_matrix {
                    for _ in 0..variants[i].bias() {
                        biased_possibilities_matrix.push(i);
                    }
                }

                let mut rng = thread_rng();
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    //slight neighbour bias
                    if let Some(i) = extract_index(
                        &a2d[((lp.0 as i32 + dx) as usize, (lp.1 as i32 + dy) as usize)],
                    ) {
                        for _ in 0..(variants[i].bias().pow(3)) {
                            biased_possibilities_matrix.push(i);
                        }
                    }
                }

                a2d[lp].fill(false);
                a2d[lp].set(
                    biased_possibilities_matrix
                        [rng.gen_range(0..biased_possibilities_matrix.len())],
                    true,
                );
            }
        }

        let mut nv = Vec::with_capacity(width * height);
        for x in 1..=width {
            for y in 1..=height {
                let index = extract_index(&a2d[(x, y)]).unwrap();
                nv.push(variants[index].clone());
            }
        }
        nv
    }
}
