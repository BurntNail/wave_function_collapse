use enum_derive_list::AllVariants;
use piston_window::{
    rectangle, Button, Key, MouseButton, PistonWindow, PressEvent, RenderEvent, Transformed,
    WindowSettings,
};
use std::collections::HashMap;
use wave_function_collapse::{coords_to_index, WFCGenerator, WFCState};

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

impl TerrainExample {
    pub fn get_colour(&self) -> [f32; 4] {
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
    const SIZE: usize = 75;
    const SCALE: usize = 15;

    let mut generator: WFCGenerator<TerrainExample> = WFCGenerator::new(SIZE, SIZE);
    let mut finished = false;

    let mut win: PistonWindow =
        WindowSettings::new("WFC", [(SIZE * SCALE) as u32, (SIZE * SCALE) as u32])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .expect("unable to make window");

    let mut drawing = generator.get_current();
    let mut changed = false;

    while let Some(e) = win.next() {
        if let Some(_r) = e.render_args() {
            if changed {
                win.draw_2d(&e, |c, gl, _device| {
                    let (width, height) = match c.viewport.iter().next() {
                        None => {
                            eprintln!("Couldn't get viewport!");
                            (0.0, 0.0)
                        }
                        Some(vp) => (vp.window_size[0], vp.window_size[1]),
                    };
                    let cell_width = width / SIZE as f64;
                    let cell_height = height / SIZE as f64;
                    let rect = [0.0, 0.0, cell_width, cell_height];

                    for y in 0..SIZE {
                        for x in 0..SIZE {
                            let xpos = y as f64 * cell_width;
                            let ypos = x as f64 * cell_height;
                            let trans = c.transform.trans(xpos, ypos);

                            rectangle(
                                drawing[coords_to_index(SIZE, (x, y))]
                                    .map(|x| x.get_colour())
                                    .unwrap_or_default(), //TODO: use the finish method
                                rect,
                                trans,
                                gl,
                            );
                        }
                    }
                });
            }
        }
        if finished {
            continue;
        }

        if let Some(Button::Mouse(btn)) = e.press_args() {
            match btn {
                MouseButton::Left => {
                    changed = true;
                    finished = generator.step();
                    drawing = generator.get_current();
                }
                MouseButton::Right => {
                    changed = true;

                    for _ in 0..100 {
                        if generator.step() {
                            finished = true;
                            println!("done");
                            break;
                        }
                    }

                    drawing = generator.get_current();
                }
                _ => {}
            }
        }

        if matches!(e.press_args(), Some(Button::Keyboard(Key::F))) {
            loop {
                if generator.step() {
                    break;
                }
            }
            finished = true;
            drawing = generator.get_current();
        }
    }

    // println!("{map:?}");
}
