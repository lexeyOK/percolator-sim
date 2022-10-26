use image::{ImageBuffer, Rgb};
use rand::{self, Rng};

use percolator_sim::Grid;

const WIDTH: usize = 1024 * 2;
const HIGHT: usize = 1024;
const FRACT: f64 = 0.5;


fn main() {
  println!("{}", WIDTH * HIGHT);
  let grid = init_random();

  let field = grid.find_connected_components();

  let image = ImageBuffer::from_fn(WIDTH as u32, HIGHT as u32, |x, y| {
      let val = (field[y as usize][x as usize]) % 256;
      Rgb([(13 * val) as u8, (17 * val) as u8, (15 * val) as u8])
  });
  image.save("output.png").unwrap();
}

fn init_random() -> Grid {
  let mut rng = rand::thread_rng();
  let mut right = vec![vec![false;WIDTH+1];HIGHT+1];
  let mut down = vec![vec![false;WIDTH+1];HIGHT+1];
  for x in 1..WIDTH {
      for y in 1..HIGHT {
          right[y][x] = rng.gen_bool(FRACT);
          down[y][x] = rng.gen_bool(FRACT);
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

