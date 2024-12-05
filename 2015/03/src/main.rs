use std::collections::HashSet;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut x = 0;
    let mut y = 0;
    let mut visited = HashSet::new();

    for c in input.trim().chars() {
        match c {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => panic!(),
        }
        visited.insert((x, y));
    }

    println!("{} part 1: {}", title, visited.len());

    let mut x0 = 0;
    let mut y0 = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut visited = HashSet::new();

    visited.insert((0, 0));

    for mut c in &input.trim().chars().chunks(2) {
        let c0 = c.next().unwrap();
        let c1 = c.next().unwrap();
        match c0 {
            '<' => x0 -= 1,
            '>' => x0 += 1,
            '^' => y0 -= 1,
            'v' => y0 += 1,
            _ => panic!(),
        }
        match c1 {
            '<' => x1 -= 1,
            '>' => x1 += 1,
            '^' => y1 -= 1,
            'v' => y1 += 1,
            _ => panic!(),
        }
        visited.insert((x0, y0));
        visited.insert((x1, y1));
    }

    println!("{} part 2: {}", title, visited.len());
}

fn main() {
    run("demo", "^v^v^v^v^v");
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
