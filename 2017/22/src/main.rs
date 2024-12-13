// Part 1: 8 mins
// Part 1+2: 16 mins

use std::collections::HashMap;

fn run(title: &str, input: &str) {
    let data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut grid = HashMap::new();
    let (w, h) = (data[0].len() as isize, data.len() as isize);
    for y in 0..h {
        for x in 0..w {
            grid.insert((x - w/2, y - h/2), data[y as usize][x as usize]);
        }
    }

    // println!("{:?}", grid);

    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, -1);

    let mut part1 = 0;
    for i in 0..10_000 {
        if *grid.get(&(x, y)).unwrap_or(&false) {
            (dx, dy) = (-dy, dx);
            grid.insert((x, y), false);
        } else {
            (dx, dy) = (dy, -dx);
            grid.insert((x, y), true);
            part1 += 1;
        }
        x += dx;
        y += dy;
    }

    println!("{} part 1: {}", title, part1);
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn run2(title: &str, input: &str) {
    let data: Vec<Vec<State>> = input
        .lines()
        .map(|line| line.chars().map(|c| if c == '#' { State::Infected } else { State::Clean }).collect())
        .collect();

    let mut grid = HashMap::new();
    let (w, h) = (data[0].len() as isize, data.len() as isize);
    for y in 0..h {
        for x in 0..w {
            grid.insert((x - w/2, y - h/2), data[y as usize][x as usize]);
        }
    }

    // println!("{:?}", grid);

    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, -1);

    let mut part2 = 0;
    for i in 0..10_000_000 {
    // for i in 0..100 {
        let c = *grid.get(&(x, y)).unwrap_or(&State::Clean);
        let n;
        (dx, dy, n) = match c {
            State::Clean => (dy, -dx, State::Weakened),
            State::Weakened => (dx, dy, State::Infected),
            State::Infected => (-dy, dx, State::Flagged),
            State::Flagged => (-dx, -dy, State::Clean),
        };
        grid.insert((x, y), n);
        if n == State::Infected {
            part2 += 1;
        }
        x += dx;
        y += dy;
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "..#
#..
...
";

fn main() {
    // run("demo", INPUT_DEMO);
    run2("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("22/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("22/input.txt").unwrap());
}
