// Part 1: 2 mins
// Part 1+2: 5 mins

use std::collections::HashSet;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    for (i, (a, b, c, d)) in input.trim().chars().tuple_windows().enumerate() {
        if HashSet::from([a, b, c, d]).len() == 4 {
            println!("{} part 1: {}", title, i + 4);
            break;
        }
    }

    for (i, cs) in input.trim().chars().collect_vec().windows(14).enumerate() {
        if HashSet::<char>::from_iter(cs.iter().copied()).len() == 14 {
            println!("{} part 2: {}", title, i + 14);
            break;
        }
    }
}

const INPUT_DEMO: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
