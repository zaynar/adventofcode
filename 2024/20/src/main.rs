// Part 1: 18 mins
// Part 1+2: 38 mins

use std::collections::VecDeque;

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let grid = Grid::from(input);
    let start = grid.find(&'S').unwrap();
    let end = grid.find(&'E').unwrap();

    let mut dist = Grid::new_empty(grid.width() as usize, grid.height() as usize, i32::MAX);

    let mut open = VecDeque::new();
    open.push_back((end, 0));

    while let Some(((x, y), d)) = open.pop_front() {

        if *dist.get(x, y) != i32::MAX {
            continue;
        }

        dist.set(x, y, d);

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if x + dx >= 0 && x + dx < grid.width() &&
            y + dy >= 0 && y + dy < grid.height() {

                if ".S".contains(*grid.get(x + dx, y + dy)) {
                    open.push_back(((x + dx, y + dy), d + 1));
                }
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    dist.clone().for_each(|x, y, d| {
        if *d == i32::MAX {
            return;
        }

        let r = 2_i32;
        for dy in -r..=r {
            for dx in -r..=r {
                let cost = dx.abs() + dy.abs();
                if cost > r {
                    continue;
                }

                if let Some(d2) = dist.try_get(x + dx, y + dy) {
                    if *d2 != i32::MAX {
                        let saving = *d - d2;

                        // println!("{} {} = {} - {} {}, = {}, save {}", x, y, d, x+dx, y+dy, d2, saving-cost);
                        if saving - cost >= 100 {
                            part1 += 1;
                        }
                    }
                }
            }
        }

        let r = 20_i32;
        for dy in -r..=r {
            for dx in -r..=r {
                let cost = dx.abs() + dy.abs();
                if cost > r {
                    continue;
                }

                if let Some(d2) = dist.try_get(x + dx, y + dy) {
                    if *d2 != i32::MAX {
                        let saving = *d - d2;

                        // println!("{} {} = {} - {} {}, = {}, save {}", x, y, d, x+dx, y+dy, d2, saving-cost);
                        if saving - cost >= 100 {
                            part2 += 1;
                        }
                    }
                }
            }
        }
    });

    println!("{} part 1: {}", title, part1);
    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("20/input.txt").unwrap());
}
