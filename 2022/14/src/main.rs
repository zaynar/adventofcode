// Part 1: 9 mins
// Part 1+2: 12 mins

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut grid = Grid::new_empty(600, 200, '.');

    let mut minx = grid.width();
    let mut maxx = 0;
    let mut miny = grid.height();
    let mut maxy = 0;

    for line in input.lines() {
        for (a, b) in line.split(" -> ").tuple_windows() {
            let (ax, ay) = a.split_once(",").unwrap();
            let (bx, by) = b.split_once(",").unwrap();
            let ax = ax.parse().unwrap();
            let ay = ay.parse().unwrap();
            let bx = bx.parse().unwrap();
            let by = by.parse().unwrap();

            if ax == bx {
                minx = minx.min(ax);
                maxx = maxx.max(bx);
                miny = miny.min(ay).min(by);
                maxy = maxy.max(ay).max(by);
                for y in ay.min(by)..=ay.max(by) {
                    grid.set(ax, y, '#');
                }
            } else {
                miny = miny.min(ay);
                maxy = maxy.max(by);
                minx = minx.min(ax).min(bx);
                maxx = maxx.max(ax).max(bx);
                for x in ax.min(bx)..=ax.max(bx) {
                    grid.set(x, ay, '#');
                }
            }
        }
    }

    let mut part1 = 0;
    loop {
        let (mut x, mut y) = (500, 0);

        while y < maxy {
            if *grid.get(x, y+1) == '.' {
                y += 1;
            } else if *grid.get(x-1, y+1) == '.' {
                x -= 1;
                y += 1;
            } else if *grid.get(x+1, y+1) == '.' {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        if y == maxy {
            break;
        }

        grid.set(x, y, 'o');
        part1 += 1;
    }

    // for y in miny-1..=maxy+1 {
    //     for x in minx-1..=maxx+1 {
    //         print!("{}", grid.get(x, y));
    //     }
    //     println!();
    // }

    println!("{} part 1: {}", title, part1);
}

fn run2(title: &str, input: &str) {
    let mut grid = Grid::new_empty(2000, 200, '.');

    let mut minx = grid.width();
    let mut maxx = 0;
    let mut miny = grid.height();
    let mut maxy = 0;

    for line in input.lines() {
        for (a, b) in line.split(" -> ").tuple_windows() {
            let (ax, ay) = a.split_once(",").unwrap();
            let (bx, by) = b.split_once(",").unwrap();
            let ax = ax.parse().unwrap();
            let ay = ay.parse().unwrap();
            let bx = bx.parse().unwrap();
            let by = by.parse().unwrap();

            if ax == bx {
                minx = minx.min(ax);
                maxx = maxx.max(bx);
                miny = miny.min(ay).min(by);
                maxy = maxy.max(ay).max(by);
                for y in ay.min(by)..=ay.max(by) {
                    grid.set(ax, y, '#');
                }
            } else {
                miny = miny.min(ay);
                maxy = maxy.max(by);
                minx = minx.min(ax).min(bx);
                maxx = maxx.max(ax).max(bx);
                for x in ax.min(bx)..=ax.max(bx) {
                    grid.set(x, ay, '#');
                }
            }
        }
    }

    let floor = 2 + maxy;

    let mut part2 = 0;
    loop {
        let (mut x, mut y) = (500, 0);

        while y < floor - 1 {
            if *grid.get(x, y+1) == '.' {
                y += 1;
            } else if *grid.get(x-1, y+1) == '.' {
                x -= 1;
                y += 1;
            } else if *grid.get(x+1, y+1) == '.' {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        grid.set(x, y, 'o');
        part2 += 1;

        if (x, y) == (500, 0) {
            break;
        }
    }

    // for y in miny-1..=maxy+1 {
    //     for x in minx-1..=maxx+1 {
    //         print!("{}", grid.get(x, y));
    //     }
    //     println!();
    // }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("14/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("14/input.txt").unwrap());
}
