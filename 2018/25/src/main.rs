use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn dist(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> i32 {
    (a.0.abs_diff(b.0) +
    a.1.abs_diff(b.1) +
    a.2.abs_diff(b.2) +
    a.3.abs_diff(b.3)) as i32
}

fn run(title: &str, input: &str) {
    let mut data: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            line.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap()
        })
        .collect();

    let mut used = HashSet::new();

    let mut part1 = 0;
    while let Some(root) = data.pop() {
        if used.contains(&root) {
            continue;
        }

        part1 += 1;

        let mut open = Vec::new();
        open.push(root);
        used.insert(root);

        while let Some(n) = open.pop() {
            for other in &data {
                if !used.contains(other) {
                    let d = dist(n, *other);
                    if d <= 3 {
                        open.push(other.clone());
                        used.insert(other.clone());
                    }
                }
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("25/input.txt").unwrap());
}
