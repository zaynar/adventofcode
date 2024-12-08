use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let w = input.lines().next().unwrap().len() as i32;
    let h = input.lines().count() as i32;

    let mut ants = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| if c == '.' { None } else { Some((x, y, c)) }).collect_vec()
    }).concat();

    let mut antmap = HashMap::new();
    for (x, y, c) in &ants {
        antmap.entry(c).or_insert_with(|| Vec::new()).push((x, y));
    }

    // println!("{:?}", antmap);

    let mut antinodes = HashMap::new();

    for (k, ants) in antmap.iter() {
        for p in ants.iter().permutations(2) {
            let (ax, ay) = (*p[0].0 as i32, *p[0].1 as i32);
            let (bx, by) = (*p[1].0 as i32, *p[1].1 as i32);
            let x = ax + (bx - ax) * 2;
            let y = ay + (by - ay) * 2;
            if x >= 0 && x < w && y >= 0 && y < h {
                antinodes.entry((x, y)).or_insert_with(|| Vec::new()).push(k);
            }
        }
    }

    // println!("{:?}", antinodes);

    println!("{} part 1: {}", title, antinodes.len());
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn run2(title: &str, input: &str) {
    let w = input.lines().next().unwrap().len() as i32;
    let h = input.lines().count() as i32;

    let mut ants = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| if c == '.' { None } else { Some((x, y, c)) }).collect_vec()
    }).concat();

    let mut antmap = HashMap::new();
    for (x, y, c) in &ants {
        antmap.entry(c).or_insert_with(|| Vec::new()).push((x, y));
    }

    let mut antinodes = HashMap::new();

    for (k, ants) in antmap.iter() {
        for p in ants.iter().permutations(2) {
            for i in 1.. {
                let (ax, ay) = (*p[0].0 as i32, *p[0].1 as i32);
                let (bx, by) = (*p[1].0 as i32, *p[1].1 as i32);
                let dx = bx - ax;
                let dy = by - ay;
                // let gcd = gcd(dx, dy).abs();
                // let dx = dx / gcd;
                // let dy = dy / gcd;

                let x = ax + dx * i;
                let y = ay + dy * i;
                if x >= 0 && x < w && y >= 0 && y < h {
                    antinodes.entry((x, y)).or_insert_with(|| Vec::new()).push(k);
                } else {
                    break;
                }
            }
        }
    }

    println!("{} part 2: {}", title, antinodes.len());
}

const INPUT_DEMO: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() {
    run("demo", INPUT_DEMO);
    run2("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
