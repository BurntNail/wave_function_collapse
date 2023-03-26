use drawing_squares::{Coloured, Window, WindowConfig};
use enum_derive_list::AllVariants;
use std::collections::HashMap;
use wave_function_collapse::WFCState;

//TODO: use images if not too much of a faff
//TODO: generating vs generated - 2 windows, close windows?
//TODO: re-generate option
//TODO: change settings (full-on egui project?)

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

impl Coloured for TerrainExample {
    fn get_colour(&self) -> [f32; 4] {
        match self {
            TerrainExample::Sand => [222.0 / 255.0, 252.0 / 255.0, 70.0 / 255.0, 1.0],
            TerrainExample::DarkSand => [125.0 / 255.0, 135.0 / 255.0, 74.0 / 255.0, 1.0],
            TerrainExample::Water => [37.0 / 255.0, 158.0 / 255.0, 146.0 / 255.0, 1.0],
            TerrainExample::DeepWater => [5.0 / 255.0, 43.0 / 255.0, 114.0 / 255.0, 1.0],
            TerrainExample::Rocks => [39.0 / 255.0, 44.0 / 255.0, 53.0 / 255.0, 1.0],
            TerrainExample::Grass => [65.0 / 255.0, 186.0 / 255.0, 52.0 / 255.0, 1.0],
            TerrainExample::Forest => [31.0 / 255.0, 102.0 / 255.0, 23.0 / 255.0, 1.0],
            TerrainExample::DeepForest => [47.0 / 255.0, 68.0 / 255.0, 35.0 / 255.0, 1.0],
        }
    }
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

fn main() {
    const SIZE: usize = 40;
    const SCALE: usize = 15;

    let mut map: Vec<TerrainExample> = WFCState::generate(SIZE, SIZE);

    let mut window = Window::new(WindowConfig::new(
        "wfc_test".into(),
        [(SIZE * SCALE) as u32, (SIZE * SCALE) as u32],
        true,
        false,
    ));

    let mut nvs = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        let mut v = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            v.push(map.remove(0));
        }
        nvs.push(v);
    }

    window.set_grid(nvs);
    window.can_continue(|_| {});

    println!("{map:?}");
}
