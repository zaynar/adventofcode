// Part 1: ?
// Part 2: 3 mins

use std::{collections::HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut part1 = 0;
    let mut part2 = 0;
    for row in input.lines() {
        let mut words = row.split_whitespace().collect_vec();
        if HashSet::<String>::from_iter(words.iter().map(|&s| s.to_owned())).len() == words.len() {
            part1 += 1;
        }

        let words = words.iter_mut().map(|w| w.chars().sorted().collect::<String>()).collect_vec();
        if HashSet::<String>::from_iter(words.clone()).len() == words.len() {
            part2 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
