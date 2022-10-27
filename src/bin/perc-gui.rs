#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::ColorImage;

use image::{ImageBuffer, Rgba};
use rand::{self, Rng};

use percolator_sim::Grid;


const WIDTH: usize = 1024;
const HIGHT: usize = 1024;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("test", options, Box::new(|_cc| Box::new(MyApp::default())));
}

struct MyApp {
    fraction: f64,
    texture: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fraction: 0.5,
            texture: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.fraction, 0.01..=1.0).text("probability"));

            let button_response = ui.button("generate");

            if button_response.clicked() {
                let grid = init_random(self.fraction);

                let field = grid.find_connected_components();

                let pixels = ImageBuffer::from_fn(WIDTH as u32, HIGHT as u32, |x, y| {
                    let val = (field[y as usize][x as usize]) % 256;
                    Rgba([(13 * val) as u8, (17 * val) as u8, (15 * val) as u8, 255])
                });
                let pixels = pixels.as_flat_samples();

                self.texture = Some(ui.ctx().load_texture(
                    "image",
                    ColorImage::from_rgba_unmultiplied([WIDTH, HIGHT], pixels.as_slice()),
                    egui::TextureFilter::Linear,
                ));
            }

            if let Some(texture) = &self.texture {
                ui.image(texture, texture.size_vec2());
            }
        });
    }
}

fn init_random(fraction: f64) -> Grid {
    let mut rng = rand::thread_rng();
    let mut right = vec![vec![false; WIDTH + 1]; HIGHT + 1];
    let mut down = vec![vec![false; WIDTH + 1]; HIGHT + 1];
    for x in 1..WIDTH {
        for y in 1..HIGHT {
            right[y][x] = rng.gen_bool(fraction);
            down[y][x] = rng.gen_bool(fraction);
        }
    }
    for y in 1..=HIGHT {
        right[y][WIDTH] = false;
    }
    for x in 1..=WIDTH {
        down[HIGHT][x] = false;
    }
    Grid::from_bools(right, down, WIDTH, HIGHT)
}
