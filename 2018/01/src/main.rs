// Part 1: 1 mins
// Part 1+2: 4 mins

use std::collections::HashSet;

fn run(title: &str, input: &str) {
    let mut n = 0;
    for x in input.lines().map(|line| line.parse::<i32>().unwrap()) {
        n += x;
    }

    println!("{} part 1: {}", title, n);
}

fn run2(title: &str, input: &str) {
    let mut n = 0;
    let mut seen = HashSet::new();
    loop {
        for x in input.lines().map(|line| line.parse::<i32>().unwrap()) {
            n += x;
            if !seen.insert(n) {
                println!("{} part 2: {}", title, n);
                return;
            }
        }
    }
}

fn main() {
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
