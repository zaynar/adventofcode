// Part 1: 13 mins
// Part 1+2: 19 mins

use std::{collections::HashSet};

use itertools::Itertools;

fn hash(input: &str) -> Vec<usize> {
    let mut data: Vec<usize> = input.trim().chars().map(|c| c as u8 as usize).collect_vec();
    data.append(&mut vec![17, 31, 73, 47, 23]);

    let len = 256;
    let mut curr = 0;
    let mut ss = 0;

    let mut a = (0..len).collect_vec();

    for round in 0..64 {

        for step in &data {
            // println!("{:?}", a);

            let mut b = a.clone();
            for i in 0..*step {
                b[(curr + i) % len] = a[(curr + len + step-1 - i) % len];
            }
            curr += step + ss;
            ss += 1;
            a = b;

            // println!("{:?}\n", a);
        }
    }

    let hash = (0..16).map(|i| {
        let mut n = 0;
        for j in 0..16 {
            n ^= a[i*16 + j];
        }
        vec![(n >> 4) & 15, n & 15]
    }).concat();

    hash
}


fn run(title: &str, input: &str) {
    let mut part1 = 0;
    let mut grid = HashSet::new();
    for r in 0..128 {
        let h = hash(format!("{}-{}", input, r).as_str());
        // println!("{:?}", h);
        for x in 0..128 {
            let n = h[(x / 4) as usize] & (1 << (3 - (x % 4)));
            if n != 0 {
                // print!("#");
                part1 += 1;
                grid.insert((x as i32, r as i32));
            } else {
                // print!(".");
            }
        }
        // println!();
    }

    println!("{} part 1: {}", title, part1);

    let mut groups = 0;
    let mut open: Vec<(i32, i32)> = Vec::new();
    let mut seen = HashSet::new();

    while let Some(root) = grid.iter().find(|(x, y)| !seen.contains(&(*x, *y)))
    {
        groups += 1;
        open.push(*root);

        while let Some((x, y)) = open.pop() {
            if !seen.insert((x, y)) {
                continue;
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if grid.contains(&(x + dx, y + dy)) {
                    open.push((x + dx, y + dy));
                }
            }
        }
    }

    println!("{} part 2: {}", title, groups);
}

fn main() {
    run("demo", "flqrgnkx");
    run("input", "xlqgujun");
}
