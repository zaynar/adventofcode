use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| str::parse(n).unwrap())
                .collect()
        })
        .collect();

    let s0 = data.iter().map(|v| v[0]).sorted();
    let s1 = data.iter().map(|v| v[1]).sorted();
    println!(
        "{} part 1: {}",
        title,
        s0.clone()
            .zip(s1.clone())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
    );

    let counts = s1.fold(HashMap::new(), |mut map, n| {
        *map.entry(n).or_insert(0) += 1;
        map
    });
    println!(
        "{} part 2: {}",
        title,
        s0.map(|n| n * counts.get(&n).unwrap_or(&0)).sum::<i32>()
    );
}

const INPUT_DEMO: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
