// Part 1: 5 mins
// Part 1+2: 9 mins

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let mut grid = Grid::new_empty(1000, 1000, 0);

    for line in input.lines() {
        let (src, dst) = line.split_once(" -> ").unwrap();
        let (sx, sy) = src.split_once(",").unwrap();
        let (dx, dy) = dst.split_once(",").unwrap();
        let sx: i32 = sx.parse().unwrap();
        let sy: i32 = sy.parse().unwrap();
        let dx: i32 = dx.parse().unwrap();
        let dy: i32 = dy.parse().unwrap();

        // if sx == dx {
        //     let y0 = i32::min(sy, dy);
        //     let y1 = i32::max(sy, dy);
        //     for y in y0..=y1 {
        //         *grid.get_mut(sx, y) += 1;
        //     }
        // } else if sy == dy {
        //     let x0 = i32::min(sx, dx);
        //     let x1 = i32::max(sx, dx);
        //     for x in x0..=x1 {
        //         *grid.get_mut(x, sy) += 1;
        //     }
        // }

        let vx = (dx - sx).signum();
        let vy = (dy - sy).signum();

        let (mut x, mut y) = (sx, sy);
        *grid.get_mut(x, y) += 1;
        while (x, y) != (dx, dy) {
            if x != dx { x += vx; }
            if y != dy { y += vy; }
            *grid.get_mut(x, y) += 1;
        }
    }

    // println!("{}", grid);
    let mut part1 = 0;
    grid.for_each(|x, y, n| if *n > 1 { part1 += 1; });

    println!("{} part N: {}", title, part1);
}

const INPUT_DEMO: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
