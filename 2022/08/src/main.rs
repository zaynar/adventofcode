// Part 1: 6 mins
// Part 1+2: 15 mins

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let grid = Grid::from(input).map(|c| c.to_digit(10).unwrap());

    let mut part1 = 0;
    grid.for_each(|x, y, c| {
        if
            (0..x).all(|nx| grid.get(nx, y) < c) ||
            (x+1..grid.width()).all(|nx| grid.get(nx, y) < c) ||
            (0..y).all(|ny| grid.get(x, ny) < c) ||
            (y+1..grid.height()).all(|ny| grid.get(x, ny) < c)
        {
            part1 += 1;
        }
    });

    let mut part2 = 0;
    grid.for_each(|x, y, c| {
        let x0 = (0..x).rev().position(|nx| grid.get(nx, y) >= c).and_then(|i| Some(i+1)).unwrap_or(x as usize);
        let x1 = (x+1..grid.width()).position(|nx| grid.get(nx, y) >= c).and_then(|i| Some(i+1)).unwrap_or((grid.width() - x - 1) as usize);
        let y0 = (0..y).rev().position(|ny| grid.get(x, ny) >= c).and_then(|i| Some(i+1)).unwrap_or(y as usize);
        let y1 = (y+1..grid.height()).position(|ny| grid.get(x, ny) >= c).and_then(|i| Some(i+1)).unwrap_or((grid.height() - y - 1) as usize);
        // println!("{} {}  {} {} {} {}", x, y, x0,x1,y0,y1);
        part2 = part2.max(x0*x1*y0*y1);
    });

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "30373
25512
65332
33549
35390
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
