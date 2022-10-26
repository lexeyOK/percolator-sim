use disjoint_sets::UnionFind;
use std::cmp::{max, min};

const CHARS: &str = "⋅╶╷┌╴─┐┬╵└│├┘┴┤┼";

pub struct Grid {
    right: Vec<Vec<bool>>,
    down: Vec<Vec<bool>>,
    pub width: usize,
    pub hight: usize,
}

impl Grid {
    pub fn new(width: usize, hight: usize) -> Grid {
        Grid {
            right: vec![vec![false; width + 1]; hight + 1],
            down: vec![vec![false; width + 1]; hight + 1],
            width,
            hight,
        }
    }

    pub fn from_bools(right: Vec<Vec<bool>>,down: Vec<Vec<bool>>, width: usize, hight: usize) -> Grid{
        Grid {
            right,
            down,
            width,
            hight,
        }
    }

    pub fn find_connected_components(&self) -> Vec<Vec<usize>> {
        let mut field = vec![vec![0; self.width]; self.hight];
        if self.width == 0 || self.hight == 0 {
            return field;
        }
        let mut next_lable = 1;
        let mut eq_set: UnionFind<usize> = UnionFind::new(self.width * self.hight);
        for x in 1..=self.width {
            for y in 1..=self.hight {
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

        let mut output_lables = vec![0usize; self.width * self.hight];
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.hight {
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

    fn draw_grid(&self) {
        for y in 1..=self.hight {
            for x in 1..=self.width {
                let mask = 8 * self.down[y - 1][x] as usize
                    + 4 * self.right[y][x - 1] as usize
                    + 2 * self.down[y][x] as usize
                    + self.right[y][x] as usize;
                print!("{}", CHARS.chars().nth(mask).unwrap_or('E'))
            }
            println!();
        }
      }
}

