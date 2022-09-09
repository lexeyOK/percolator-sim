use std::cmp::{max, min};

use disjoint_sets::UnionFind;
use image::{ImageBuffer, Rgb};
use rand::{self, Rng};
const WIDTH: usize = 1024 * 3;
const HIGHT: usize = 1024;
const FRACT: f64 = 0.5;
const CHARS: &str = "⋅╶╷┌╴─┐┬╵└│├┘┴┤┼";

fn main() -> ! {
    //println!("{}", WIDTH * HIGHT);
    let mut grid = Grid::new();
    loop {
        grid.init_random();
        let field = grid.find_connected_components();
        let image = ImageBuffer::from_fn(WIDTH as u32, HIGHT as u32, |x, y| {
            let val = field[(y as usize) * HIGHT + (x as usize)] % 256;
            Rgb([(13 * val) as u8, (17 * val) as u8, (15 * val) as u8])
        });
        //let image: ImageBuffer<Luma<_>, Vec<_>> =
        //    ImageBuffer::from_fn(WIDTH as u32, HIGHT as u32, |x, y| {
        //        Luma::<u16>((field(y as usize)(x as usize).try_into().unwrap()))
        //    });
        //let image = image::imageops::rotate270(&image);
        image.save("output.png").unwrap();
    }
}

struct Grid {
    right: Vec<bool>,
    down: Vec<bool>,
}

impl Grid {
    fn draw_grid(&self) {
        for y in 1..=HIGHT {
            for x in 1..=WIDTH {
                let mask = 8 * self.down[(y - 1) * (HIGHT + 1) + x] as usize
                    + 4 * self.right[(y) * (HIGHT + 1) + x - 1] as usize
                    + 2 * self.down[(y) * (HIGHT + 1) + (x)] as usize
                    + self.right[(y) * (HIGHT + 1) + (x)] as usize;
                print!("{}", CHARS.chars().nth(mask).unwrap_or('E'))
            }
            println!();
        }
    }

    fn find_connected_components(&self) -> Vec<usize> {
        let mut field = vec![0; WIDTH * HIGHT];
        if WIDTH == 0 || HIGHT == 0 {
            return field;
        }
        let mut next_lable = 1;
        let mut eq_set: UnionFind<usize> = UnionFind::new(WIDTH * HIGHT);
        for x in 1..=WIDTH {
            for y in 1..=HIGHT {
                let (nodes, num) = {
                    let mut temp = (0, 0);
                    let mut num = 0;
                    if self.down[(y - 1) * (HIGHT + 1) + (x)] {
                        //println!("{} {}", y, x);
                        temp.0 = field[y * HIGHT + x - 2 * HIGHT - 1];
                        num += 1;
                    }
                    if self.right[(y) * (HIGHT + 1) + (x - 1)] {
                        //println!("{} {}",y,x);
                        temp.1 = field[y * HIGHT + x - HIGHT - 2];
                        num += 1;
                    }
                    (temp, num)
                };
                match num {
                    0 => {
                        field[(y - 1) * HIGHT + (x - 1)] = next_lable;
                        next_lable += 1;
                    }
                    1 => {
                        field[(y - 1) * HIGHT + (x - 1)] = max(nodes.0, nodes.1);
                    }
                    2 => {
                        field[(y - 1) * HIGHT + (x - 1)] = min(nodes.0, nodes.1);
                        eq_set.union(nodes.0, nodes.1);
                    }
                    _ => unreachable!(),
                }
            }
        }

        let mut output_lables = vec![0usize; WIDTH * HIGHT];
        let mut count = 0;
        for x in 0..WIDTH {
            for y in 0..HIGHT {
                let label = field[(y) * HIGHT + (x)];
                let root = eq_set.find(label);
                let mut output_lable = output_lables[root];
                if output_lable < 1 {
                    output_lable = count;
                    count += 1;
                }
                output_lables[root] = output_lable;
                field[(y) * HIGHT + (x)] = output_lable;
            }
        }
        field
    }

    fn init_random(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 1..=WIDTH {
            for y in 1..=HIGHT {
                self.right[(y) * (HIGHT + 1) + (x)] = rng.gen_bool(FRACT);
                self.down[(y) * (HIGHT + 1) + (x)] = rng.gen_bool(FRACT);
            }
        }
        for y in 1..=HIGHT {
            self.right[(y) * (HIGHT + 1) + (WIDTH)] = false;
        }
        for x in 1..=WIDTH {
            self.down[(HIGHT) * (HIGHT + 1) + (x)] = false;
        }
    }

    fn new() -> Grid {
        Grid {
            right: vec![false; (WIDTH + 1) * (HIGHT + 1)],
            down: vec![false; (WIDTH + 1) * (HIGHT + 1)],
        }
    }
}
