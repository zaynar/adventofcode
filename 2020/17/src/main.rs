// Part 1: 11 mins
// Part 1+2: 16 mins

use std::collections::{HashMap, HashSet};

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut grid: HashMap<(i32, i32, i32), bool> = HashMap::new();

    Grid::from(input).for_each(|x, y, c| {
        if *c == '#' {
            grid.insert((x, y, 0), true);
        }
    });

    let reps = 6;
    let range = 16 + reps;

    for rep in 0..reps {
        let mut new = HashMap::new();

        for z in -range..range {
            for y in -range..range {
                for x in -range..range {

                    let n = (0..3).map(|_| -1..=1).multi_cartesian_product().filter(|d| {
                        let (dx, dy, dz) = (d[0], d[1], d[2]);
                        (dx, dy, dz) != (0, 0, 0) && grid.get(&(x+dx, y+dy, z+dz)) == Some(&true)
                    }).count();

                    if grid.get(&(x, y, z)) == Some(&true) {
                        if n == 2 || n == 3 {
                            new.insert((x, y, z), true);
                        }
                    } else {
                        if n == 3 {
                            new.insert((x, y, z), true);
                        }
                    }
                }
            }
        }

        grid = new;
    }

    // println!("{:?}", grid);

    println!("{} part 1: {}", title, grid.len());
}

fn run2(title: &str, input: &str) {
    let mut grid: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    Grid::from(input).for_each(|x, y, c| {
        if *c == '#' {
            grid.insert((x-4, y-4, 0, 0));
        }
    });

    let reps = 6;
    let range = 5 + reps;

    for rep in 0..reps {
        println!("{} {}", rep, grid.len());
        let mut new = HashSet::new();

        for w in -range..range {
            for z in -range..range {
                for y in -range..range {
                    for x in -range..range {

                        let n = (0..4).map(|_| -1..=1).multi_cartesian_product().filter(|d| {
                            let (dx, dy, dz, dw) = (d[0], d[1], d[2], d[3]);
                            (dx, dy, dz, dw) != (0, 0, 0, 0) && grid.contains(&(x+dx, y+dy, z+dz, w+dw))
                        }).count();

                        if grid.contains(&(x, y, z, w)) {
                            if n == 2 || n == 3 {
                                new.insert((x, y, z, w));
                            }
                        } else {
                            if n == 3 {
                                new.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }

        grid = new;
    }

    // println!("{:?}", grid);

    println!("{} part 2: {}", title, grid.len());
}

const INPUT_DEMO: &str = ".#.
..#
###
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("17/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("17/input.txt").unwrap());
}
