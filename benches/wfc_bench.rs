use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use wave_function_collapse::WFCState;

#[derive(AllVariants, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum TerrainExample {
    Sand,
    DarkSand,
    Water,
    DeepWater,
    Rocks,
    Grass,
    Forest,
    DeepForest,
}

impl WFCState for TerrainExample {
    fn get_variants() -> &'static [Self] {
        TerrainExample::all_variants()
    }

    fn to_usize(self) -> usize {
        self as usize
    }

    fn bias(&self) -> usize {
        match self {
            TerrainExample::Sand => 2,
            TerrainExample::DarkSand => 1,
            TerrainExample::Water => 2,
            TerrainExample::DeepWater => 3,
            TerrainExample::Rocks => 1,
            TerrainExample::Grass => 2,
            TerrainExample::Forest => 3,
            TerrainExample::DeepForest => 1,
        }
    }

    fn possible_neighbours() -> HashMap<Self, Vec<Self>> {
        use TerrainExample::{DarkSand, DeepForest, DeepWater, Forest, Grass, Rocks, Sand, Water};

        HashMap::from([
            (Sand, vec![Water, Grass]),
            (DarkSand, vec![Sand, Water]),
            (Water, vec![DeepWater, Forest]),
            (DeepWater, vec![Water, Rocks]),
            (Rocks, vec![DeepWater, DeepForest]),
            (Grass, vec![Sand, Forest]),
            (Forest, vec![Grass, DeepForest, Water]),
            (DeepForest, vec![Rocks, Forest]),
        ])
    }
}

fn bench_terrain_example(c: &mut Criterion) {
    c.bench_function("wfc size 20", |b| {
        b.iter(|| TerrainExample::generate(black_box(20), black_box(20)));
    });
    c.bench_function("wfc size 10", |b| {
        b.iter(|| TerrainExample::generate(black_box(10), black_box(10)));
    });
    c.bench_function("wfc size 4", |b| {
        b.iter(|| TerrainExample::generate(black_box(4), black_box(4)));
    });
}

criterion_group!(benches, bench_terrain_example);
criterion_main!(benches);
