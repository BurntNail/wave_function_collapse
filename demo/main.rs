use enum_derive_list::AllVariants;
use std::collections::HashMap;
use std::time::Instant;
use wave_function_collapse::{coords_to_index, WFCGenerator, WFCState};
use indicatif::ProgressBar;
use image::{Rgb, ImageBuffer};
use indicatif::style::ProgressStyle;

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
    Nothing,
}

impl TerrainExample {
    pub fn get_colour(&self) -> [u8; 3] {
        match self {
            TerrainExample::Sand => [222, 252, 70],
            TerrainExample::DarkSand => [125, 135, 74],
            TerrainExample::Water => [37, 158, 146],
            TerrainExample::DeepWater => [5, 43, 114],
            TerrainExample::Rocks => [39, 44, 53],
            TerrainExample::Grass => [65, 186, 52],
            TerrainExample::Forest => [31, 102, 23],
            TerrainExample::DeepForest => [47, 68, 35],
            TerrainExample::Nothing => [0; 3],
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

    fn bias(&self, o: &Self) -> usize {
        use TerrainExample::*;

        if !Self::possible_neighbours()[self].contains(o) {
            return 0;
        }

        match (self, o) {
            //sand
            (Sand | DarkSand, Sand | DarkSand) => 2,
            (Sand, Water) | (Water, Sand) | (Sand, Grass) | (Grass, Sand) => 1,
            //darksand
            (DarkSand, Water) | (Water, DarkSand) => 2,
            //water
            (Water, Water) => 3,
            (Water, Forest) | (Forest, Water) => 1,
            (Water, DeepWater) | (DeepWater, Water) => 2,
            //deep water
            (DeepWater, DeepWater) => 5,
            //rocks
            (DeepWater | DeepForest, Rocks) | (Rocks, DeepWater | DeepForest) => 1,
            //grass
            (Grass, Grass) => 2,
            (Grass, Forest) | (Forest, Grass) => 3,
            //forest
            (Forest, Forest) => 4,
            (Forest, DeepForest) | (DeepForest, Forest) => 3,
            //deep forest
            (DeepForest, DeepForest) => 5,

            _ => unimplemented!("{self:?} vs {o:?}"),
        }
    }

    fn possible_neighbours() -> HashMap<Self, Vec<Self>> {
        use TerrainExample::{DarkSand, DeepForest, DeepWater, Forest, Grass, Rocks, Sand, Water};

        HashMap::from([
            (Sand, vec![Sand, Water, Grass]),
            (DarkSand, vec![DarkSand, Sand, Water]),
            (Water, vec![Water, Sand, DeepWater, Forest]),
            (DeepWater, vec![DeepWater, Water, Rocks]),
            (Rocks, vec![DeepWater, DeepForest]),
            (Grass, vec![Grass, Sand, Forest]),
            (Forest, vec![Forest, Grass, DeepForest, Water]),
            (DeepForest, vec![DeepForest, Rocks, Forest]),
        ])
    }

    fn none_option() -> Self {
        Self::Nothing
    }
}

fn main() {
    const SIZE: usize = 250;

    let mut generator: WFCGenerator<TerrainExample> = WFCGenerator::new(SIZE, SIZE);
    let mut finished = false;
    let bar = ProgressBar::new((SIZE * SIZE) as u64).with_style(ProgressStyle::with_template("{spinner} Elapsed: [{elapsed_precise}], ETA: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}").unwrap());

    let mut buff = ImageBuffer::new(SIZE as u32, SIZE as u32);

    for px in buff.pixels_mut() {
        *px = Rgb::from([0, 0, 0])
    }

    println!("Saving to image");

    let mut i = 0;
    while !finished {
        for _ in 0..10 {
            finished = generator.step_moar_random();
            i += 1;
        }
        bar.inc(10);

        for (terrain, pixel) in generator.get_current().into_iter().zip(buff.pixels_mut()).filter_map(|(terrain, px)| terrain.map(|terrain| (terrain, px))) {
            *pixel = Rgb::from(terrain.get_colour());
        }
        buff.save(format!("imgs/out_{i}.png")).unwrap();
    }

    bar.finish();

    println!("Maybe finished?");


    let finished = generator.finish();


    for (terrain, pixel) in finished.into_iter().zip(buff.pixels_mut()) {
        *pixel = Rgb::from(terrain.get_colour())
    }

    buff.save("out.png").unwrap();
}
