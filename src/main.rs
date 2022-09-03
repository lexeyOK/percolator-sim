use std::{
    cmp::{max, min},
    fs::File,
};

use disjoint_sets::UnionFind;
use image::codecs::gif::{GifEncoder};
use image::{Frame,imageops::rotate270, ImageBuffer, Rgba};
use rand::{self, Rng};
const WIDTH: usize = 1024 * 3;
const HIGHT: usize = 1024 * 2;
const FRACT: f64 = 0.5;
const CHARS: &str = "⋅╶╷┌╴─┐┬╵└│├┘┴┤┼";

fn main() {
    println!("{}", WIDTH * HIGHT);
    let mut fraction = 1.0;
    let mut grid = Grid::new();
    let mut file_out = File::open("out.gif").unwrap();
    let mut encoder = GifEncoder::new(file_out);
    let mut frames = vec![];
    loop {
        fraction -= 0.1;
        if fraction < 0.1 {
            break;
        }
        grid.init_random(fraction);
        // grid.draw_grid();
        let field = grid.find_connected_components();
        frames.push(Frame::new(ImageBuffer::from_fn(WIDTH as u32, HIGHT as u32, |x, y| {
            let val = (field[y as usize][x as usize]) % 256;
            Rgba([(13 * val) as u8, (17 * val) as u8, (15 * val) as u8,255])
        })));
        //let image: ImageBuffer<Luma<_>, Vec<_>> =
        //    ImageBuffer::from_fn(WIDTH as u32, HIGHT as u32, |x, y| {
        //        Luma::<u16>([field[y as usize][x as usize].try_into().unwrap()])
        //    });
        //let image = rotate270(&image);
    }
    
    encoder.encode_frames(frames).unwrap();
}

struct Grid {
    right: Vec<Vec<bool>>,
    down: Vec<Vec<bool>>,
}

impl Grid {
    fn draw_grid(&self) {
        for y in 1..=HIGHT {
            for x in 1..=WIDTH {
                let mask = 8 * self.down[y - 1][x] as usize
                    + 4 * self.right[y][x - 1] as usize
                    + 2 * self.down[y][x] as usize
                    + self.right[y][x] as usize;
                print!("{}", CHARS.chars().nth(mask).unwrap_or('E'))
            }
            println!();
        }
    }

    fn find_connected_components(&self) -> Vec<Vec<usize>> {
        let mut field = vec![vec![0; WIDTH]; HIGHT];
        if WIDTH == 0 || HIGHT == 0 {
            return field;
        }
        let mut next_lable = 1;
        let mut eq_set: UnionFind<usize> = UnionFind::new(WIDTH * HIGHT);
        for x in 1..=WIDTH {
            for y in 1..=HIGHT {
                // get neighbours
                let (nodes, num) = {
                    let mut temp = (0, 0);
                    let mut num = 0;
                    if self.down[y - 1][x] {
                        temp.0 = field[y - 2][x - 1];
                        num += 1;
                    }
                    if self.right[y][x - 1] {
                        temp.1 = field[y - 1][x - 2];
                        num += 1;
                    }
                    (temp, num)
                };
                match num {
                    0 => {
                        field[y - 1][x - 1] = next_lable;
                        next_lable += 1;
                    }
                    1 => {
                        field[y - 1][x - 1] = max(nodes.0, nodes.1);
                    }
                    2 => {
                        field[y - 1][x - 1] = min(nodes.0, nodes.1);
                        eq_set.union(nodes.0, nodes.1);
                    }
                    _ => unreachable!(),
                }
            }
        }
        //println!("{:?}", (0..WIDTH*HIGHT).map(|n| eq_set.find(n)).collect::<Vec<_>>());
        let mut output_lables = vec![0usize; WIDTH * HIGHT];
        let mut count = 0;
        for x in 0..WIDTH {
            for y in 0..HIGHT {
                let label = field[y][x];
                let root = eq_set.find(label);
                let mut output_lable = output_lables[root];
                if output_lable < 1 {
                    output_lable = count;
                    count += 1;
                }
                output_lables[root] = output_lable;
                field[y][x] = output_lable;
            }
        }
        field
    }

    fn init_random(&mut self, fraction: f64) {
        let mut rng = rand::thread_rng();
        for x in 1..=WIDTH {
            for y in 1..=HIGHT {
                self.right[y][x] = rng.gen_bool(fraction);
                self.down[y][x] = rng.gen_bool(fraction);
            }
        }
        for y in 1..=HIGHT {
            self.right[y][WIDTH] = false;
        }
        for x in 1..=WIDTH {
            self.down[HIGHT][x] = false;
        }
    }

    fn new() -> Grid {
        Grid {
            right: vec![vec![false; WIDTH + 1]; HIGHT + 1],
            down: vec![vec![false; WIDTH + 1]; HIGHT + 1],
        }
    }
}
