use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn evolve(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let w = 5;
    let h = 5;
    (0..h).map(|y| {
        (0..w).map(|x| {
            let mut n = 0;
            if x > 0 && grid[y][x-1] { n += 1; }
            if x < w-1 && grid[y][x+1] { n += 1; }
            if y > 0 && grid[y-1][x] { n += 1; }
            if y < h-1 && grid[y+1][x] { n += 1; }
            if grid[y][x] {
                n == 1
            } else {
                n == 1 || n == 2
            }
        }).collect()
    }).collect()
}

fn weight(grid: &Vec<Vec<bool>>) -> u32 {
    let w = 5;
    let h = 5;
    (0..h).map(|y| {
        (0..w).map(|x| {
            if grid[y][x] { 1 << (x + y * w) } else { 0 }
        }).sum::<u32>()
    }).sum()
}

fn get(grid: u32, x: u32, y: u32) -> bool {
    (grid & (1 << (x + y * 5))) != 0
}

fn evolve2(grid: u32) -> u32 {
    let w = 5;
    let h = 5;
    (0..h).map(|y| {
        (0..w).map(|x| {
            let i = 1 << (x + y * 5);

            let mut n = 0;
            if x > 0 && get(grid, x-1, y) { n += 1; }
            if x < w-1 && get(grid, x+1, y) { n += 1; }
            if y > 0 && get(grid, x, y-1) { n += 1; }
            if y < h-1 && get(grid, x, y+1) { n += 1; }
            let alive = if get(grid, x, y) {
                n == 1
            } else {
                n == 1 || n == 2
            };
            if alive { i } else { 0 }
        }).sum::<u32>()
    }).sum()
}


fn evolve3(grids: &HashMap<i32, u32>) -> HashMap<i32, u32> {

    let mut new_grids = HashMap::new();

    let min_level = grids.keys().min().unwrap();
    let max_level = grids.keys().max().unwrap();

    for level in (min_level-1)..=(max_level+1) {
        let mut new_grid = 0;

        for i in 0..25 {
            let x = i % 5;
            let y = i / 5;
            let mut neighbours = Vec::new();

            if x == 0 {
                neighbours.push((-1, 11));
            } else {
                neighbours.push((0, i - 1));
            }
            if x == 4 {
                neighbours.push((-1, 13));
            } else {
                neighbours.push((0, i + 1));
            }
            if y == 0 {
                neighbours.push((-1, 7));
            } else {
                neighbours.push((0, i - 5));
            }
            if y == 4 {
                neighbours.push((-1, 17));
            } else {
                neighbours.push((0, i + 5));
            }

            if i == 11 {
                for ny in 0..5 {
                    neighbours.push((1, 0 + ny * 5));
                }
            }
            if i == 13 {
                for ny in 0..5 {
                    neighbours.push((1, 4 + ny * 5));
                }
            }
            if i == 7 {
                for nx in 0..5 {
                    neighbours.push((1, nx));
                }
            }
            if i == 17 {
                for nx in 0..5 {
                    neighbours.push((1, 20 + nx));
                }
            }

            let alive = if (x, y) == (2, 2) {
                false
            } else {
                let n = neighbours.iter().filter(|(dl, ni)| {
                    grids.get(&(level + dl)).unwrap_or(&0) & (1 << ni) != 0
                }).count();

                // println!("level {}: {},{}: n={} {:?}", level, x, y, n, neighbours);

                if grids.get(&level).unwrap_or(&0) & (1 << i) != 0 {
                    n == 1
                } else {
                    n == 1 || n == 2
                }
            };

            if alive {
                new_grid |= 1 << i;
            }
        }

        if new_grid != 0 {
            new_grids.insert(level, new_grid);
        }
    }

    new_grids
}


fn print_grid(grid: u32) {
    let w = 5;
    let h = 5;
    for y in 0..h {
        for x in 0..w {
            print!("{}", if get(grid, x, y) { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

fn run(title: &str, input: &str) {
    let data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    println!("{:?}\n", data);

    let mut seen = HashSet::new();

    let mut grid = data;
    for i in 0.. {
        grid = evolve(&grid);
        if !seen.insert(grid.clone()) {
            println!("{} part 1: {}", title, weight(&grid));
            break;
        }
    }
}

/*

Each level is 5x5

*/

fn run2(title: &str, input: &str, reps: usize) {
    let data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

        // println!("{:?}\n", data);

    let grid0 = weight(&data);
    print_grid(grid0);

    let mut grids = HashMap::new();
    grids.insert(0, grid0);

    for i in 0..reps {
        grids = evolve3(&grids);

        // for i in grids.keys().sorted() {
        //     println!("Depth {}:", i);
        //     print_grid(*grids.get(&i).unwrap());
        // }
    }

    let num_bugs: u32 = grids.values().map(|g| g.count_ones()).sum();
    println!("{} part 2: {}", title, num_bugs);
}

const INPUT_DEMO: &str = "....#
#..#.
#..##
..#..
#....
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("24/input.txt").unwrap());
    run2("demo", INPUT_DEMO, 10);
    run2("input", &std::fs::read_to_string("24/input.txt").unwrap(), 200);
}
