// Part 1: 23 mins
// Part 1+2: 36 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let patterns = [
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(1,0), (0,1),(1,1),(2,1), (1,2)],
        vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        vec![(0,0),(0,1),(0,2),(0,3)],
        vec![(0,0),(0,1),(1,0),(1,1)],
    ];

    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    let mut placed = vec![];
    let mut height = 0;
    let (mut x, mut y) = (2, 3);
    for dir in input.replace("\n", "").trim().chars().cycle() {
        let dir = match dir {
            '<' => -1,
            '>' => 1,
            _ => panic!()
        };

        let pattern = &patterns[placed.len() % patterns.len()];
        let mut blocked = false;
        for (px, py) in pattern {
            if x + px + dir < 0 || x + px + dir >= 7 {
                blocked = true;
            }
            if grid.contains(&(x + px + dir, y + py)) {
                blocked = true;
            }
        }
        if !blocked {
            // println!("Pushed {}", dir);
            x += dir;
        } else {
            // println!("Can't push {}", dir);
        }

        let mut blocked = false;
        for (px, py) in pattern {
            if y + py - 1 < 0 {
                blocked = true;
            }
            if grid.contains(&(x + px, y + py - 1)) {
                blocked = true;
            }
        }
        if blocked {
            placed.push((x, y));

            for (px, py) in pattern {
                height = height.max(y + py);
                grid.insert((x + px, y + py));
            }

            (x, y) = (2, height + 4);

            // for y in (0..height + 2).rev() {
            //     for x in 0..9 {
            //         print!("{}", if grid.contains(&(x, y)) { '#' } else { '.' });
            //     }
            //     println!();
            // }
            // println!();
            // if placed.len() > 8 {
            //     break;
            // }

            if placed.len() == 2022 {
                println!("{} part 1: {}", title, height + 1);
                break;
            }

        } else {
            // println!("Fall");
            y -= 1;
        }
    }
}

fn run2(title: &str, input: &str) {
    let patterns = [
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(1,0), (0,1),(1,1),(2,1), (1,2)],
        vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        vec![(0,0),(0,1),(0,2),(0,3)],
        vec![(0,0),(0,1),(1,0),(1,1)],
    ];

    let mut seen = HashMap::new();

    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    let mut placed = vec![];
    let mut height = 0;
    let (mut x, mut y) = (2, 3);
    let dirs = input.replace("\n", "").trim().chars().collect_vec();
    for i in 0.. {
        let dir = match dirs[i % dirs.len()] {
            '<' => -1,
            '>' => 1,
            _ => panic!()
        };

        let pattern = &patterns[placed.len() % patterns.len()];
        let mut blocked = false;
        for (px, py) in pattern {
            if x + px + dir < 0 || x + px + dir >= 7 {
                blocked = true;
            }
            if grid.contains(&(x + px + dir, y + py)) {
                blocked = true;
            }
        }
        if !blocked {
            // println!("Pushed {}", dir);
            x += dir;
        } else {
            // println!("Can't push {}", dir);
        }

        let mut blocked = false;
        for (px, py) in pattern {
            if y + py - 1 < 0 {
                blocked = true;
            }
            if grid.contains(&(x + px, y + py - 1)) {
                blocked = true;
            }
        }
        if blocked {
            placed.push((x, y));

            for (px, py) in pattern {
                height = height.max(y + py);
                grid.insert((x + px, y + py));
            }

            (x, y) = (2, height + 4);

            // for y in (0..height + 2).rev() {
            //     for x in 0..9 {
            //         print!("{}", if grid.contains(&(x, y)) { '#' } else { '.' });
            //     }
            //     println!();
            // }
            // println!();
            // if placed.len() > 8 {
            //     break;
            // }


            let horizon = (0..7).map(|x| {
                for y in (0..=height).rev() {
                    if grid.contains(&(x, y)) {
                        return height - y;
                    }
                }
                -1
            }).collect_vec();
            // println!("{:?}", horizon);

            if let Some((pp, ph)) = seen.insert((horizon.clone(), i % dirs.len()), (placed.len(), height)) {
                let p = placed.len();
                println!("Repeat {} {}, {} {}, {:?}", p, p, ph, height, (horizon, i % dirs.len()));
                if (1000000000000 - p) % (p - pp) == 0 {
                    println!("{} part 2: {}", title,
                    height as usize +1 +
                    (1000000000000 - p) / (p - pp) * ((height - ph) as usize)
                );
                break;
                }
            }

            // if placed.len() > 5000 {
            //     break;
            // }

        } else {
            // println!("Fall");
            y -= 1;
        }
    }
}

const INPUT_DEMO: &str = "
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("17/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("17/input.txt").unwrap());
}
