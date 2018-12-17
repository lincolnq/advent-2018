use itertools::structs::Product;
use std::ops::RangeInclusive;

const SERIAL:i32 = 8868;
//const SERIAL:i32 = 42;
const GRID_SIZE:i32 = 300;
//const GRID_SIZE:i32 = 4;

fn ix2grid(ix: i32) -> (i32, i32) {
    (ix % GRID_SIZE + 1, ix / GRID_SIZE + 1)
}

fn grid2ix(gridsize: i32, x: i32, y: i32) -> usize {
    (((y-1) * gridsize) + x - 1) as usize
}

fn best_square(grid: &Vec<i32>, square_size: i32) -> (i32, (i32, i32)) {
    let sqdelta = square_size - 1;
    let mut best = (0, (-1, -1));
    for (x, y) in iproduct!(1..=(GRID_SIZE - sqdelta), 1..=(GRID_SIZE - sqdelta)) {
        let total_power = iproduct!(x..=x+sqdelta, y..=y+sqdelta)
            .map(|(x,y)| {
                //println!("checking {},{}", x, y);
                grid.get(grid2ix(GRID_SIZE, x, y)).unwrap()
            })
            .sum();
        if total_power > best.0 {
            best = (total_power, (x, y));
        }
    }
    best
}

fn sum_grid(grid: &Vec<i32>, range: Product<RangeInclusive<i32>, RangeInclusive<i32>>) -> i32 {
    range.map(|(x,y)| {
            //println!("checking {},{}", x, y);
            grid.get(grid2ix(GRID_SIZE, x, y)).unwrap()
        })
        .sum()
}

fn improve_summary(grid: &Vec<i32>, prev: Vec<i32>, prev_size: i32, square_size: i32) -> Vec<i32> {
    // given a prev summary (prev_size x prev_size) and a square size, produces a new
    // summary of (prev_size + 1 x prev_size + 1) by adding the adjacent squares from 'grid'


    let square_delta = square_size - 1;
    let mut result = Vec::new();

    for (y, x) in iproduct!(1..=prev_size-1, 1..=prev_size-1) {
        let mut sum = *prev.get(grid2ix(prev_size, x, y)).unwrap();
        // assume 'sum' is the sum of the (square-1)*(square-1) squares starting at (x,y) in grid

        // lastx is the x-coordinate of the column we are adding to the sum totals
        // lasty is the y-coordinate of the row:
        let (lastx, lasty) = (x + square_delta, y + square_delta);
        sum +=
            sum_grid(grid, iproduct!(x..=lastx, lasty..=lasty)) +
                sum_grid(grid, iproduct!(lastx..=lastx, y..=(lasty - 1)));

        result.push(sum)
    }

    result
}

pub fn advent11(s: String) -> Result<String, &'static str> {

    let mut grid = Vec::new();
    for i in 0..GRID_SIZE*GRID_SIZE {
        let (x,y) = ix2grid(i);
        let rack_id = x + 10;
        let power_ones = (rack_id * y + SERIAL) * rack_id;
        let power_hundreds = (power_ones / 100) % 10;
        let power = power_hundreds - 5;
        grid.push(power)
    }

    //println!("Grid: {:?}", grid);

    let mut best = (0, (-1, -1), 0);
    let mut summary = grid.clone();
    for squaresize in 2..=GRID_SIZE {
        let new_summary_size = GRID_SIZE - squaresize + 1;
        let summary2 = improve_summary(&grid, summary, GRID_SIZE - squaresize + 2, squaresize);
        println!("s{}: {}^2", squaresize, new_summary_size);

        for (y, x) in iproduct!(1..=new_summary_size, 1..=new_summary_size) {
            let result = summary2.get(grid2ix(new_summary_size,     x, y)).unwrap();
            if *result > best.0 {
                best = (*result, (x,y), squaresize);
            }
        }

        summary = summary2;

        println!("checked {}", squaresize);
    }

    println!("Best anyxany: {:?}", best);

    Err("Nyi")
}
