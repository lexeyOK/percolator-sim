//use image::{ImageBuffer,Rgb};
use rand::{self, Rng};

const WIDTH:usize = 25;
const HIGHT:usize = 25;
const FRACT: f64 = 0.5;
const CHARS:&str  = "⋅╶╷┌╴─┐┬╵└│├┘┴┤┼";
fn main() {
    let mut rng = rand::thread_rng();
    let mut right: [[bool;WIDTH+1];HIGHT+1]=[[false;WIDTH+1];HIGHT+1];
    let mut down: [[bool;WIDTH+1];HIGHT+1]=[[false;WIDTH+1];HIGHT+1];
    for x in 1..=WIDTH{
        for y in 1..=HIGHT{
            right[y][x]=rng.gen_bool(FRACT);
            down[y][x]=rng.gen_bool(FRACT);
        }
    }
    for y in 1..=WIDTH{
        right[y][WIDTH]=false;
    }
    for x in 1..=HIGHT{
        down[HIGHT][x] = false;
    }

    let mut field = [[0;WIDTH];HIGHT];
    draw_grid(&right, &down);

    /*
    let mut used= [[false;WIDTH];HIGHT];
    let mut stack = Vec::<(usize,usize)>::new();
    let mut color = 0;
    for x in 0..WIDTH{
        for y in 0..HIGHT{
            if used[x][y]{
                continue;
            }
            stack.push((x,y));
            while stack.len()!=0{
                let (a,b) = stack.pop().unwrap();
                if used[a][b]{continue;}
                field[a][b] = color;
                if a>=1 && right[a-1][b]{
                    stack.push((a-1,b));
                }
                if a+1<=WIDTH && right[a][b] {
                    stack.push((a+1,b));
                }
                if b+1<=HIGHT && down[a][b]{
                    stack.push((a,b+1));
                }
                if b>=1 && down[a][b-1]{
                    stack.push((a,b-1));
                }
                used[a][b]=true;
            }
            color+=1;
        }
    }
    */


}

fn draw_grid(right:&[[bool;WIDTH+1];HIGHT+1],down:&[[bool;WIDTH+1];HIGHT+1]){
    for y in 1..=HIGHT{
        for x in 1..=WIDTH{
            let mask = 
            8 * down[y-1][x] as usize+
            4 * right[y][x-1] as usize+
            2 * down[y][x] as usize+
            right[y][x] as usize;
            print!("{}",CHARS.chars().nth(mask).unwrap_or('E')) 
        }
        println!("");
    }
}