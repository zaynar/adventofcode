// Part 1: 6 mins
// Part 1+2: 20 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn secret(s: i64) -> i64 {
    let n = ((s * 64) ^ s) % (1 << 24);
    let n = ((n / 32) ^ n) % (1 << 24);
    let n = ((n * 2048) ^ n) % (1 << 24);
    n
}

fn run(title: &str, input: &str) {

    let mut part1 = 0;
    for line in input.lines() {
        let n = line.parse().unwrap();

        let mut s = n;
        for i in 0..2000 {
            s = secret(s);
        }
        // println!("{}: {}", n, s);
        part1 += s;
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, "TODO");
}

fn run2(title: &str, input: &str) {

    // Each input gives 1+2000 secret numbers, 2000 deltas

    // Give seq of 4 changes
    // Want max value after the last change

    let mut scores = HashMap::new();

    for line in input.lines() {
        let n = line.parse().unwrap();

        let mut deltas = vec![];

        let mut s = n;
        for i in 0..2000 {
            let t = secret(s);
            let d = t % 10 - s % 10;
            deltas.push((d, t % 10));
            s = t;
        }

        // println!("{}: {:?}", n, deltas);

        let mut seen = HashSet::new();

        for win in deltas.windows(4) {
            let ds = win.iter().map(|(d, t)| d).copied().collect_vec();
            let score = win.last().unwrap().1;
            // println!("  {} {:?}", score, ds);

            if seen.insert(ds.clone()) {
                *scores.entry(ds).or_insert(0) += score;
            }
        }
    }

    // println!("{:#?}", scores);

    println!("{} part 2: {:?}", title, scores.iter().max_by_key(|(k, v)| *v));

}
const INPUT_DEMO: &str = "1
10
100
2024
";

const INPUT_DEMO2: &str = "1
2
3
2024
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("22/input.txt").unwrap());

    // run2("demo", "123");
    run2("demo", INPUT_DEMO2);
    run2("input", &std::fs::read_to_string("22/input.txt").unwrap());

}
