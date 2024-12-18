// Part 1: 2 mins
// Part 1+2: 4 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut has2 = 0;
    let mut has3 = 0;
    for line in input.lines() {
        let freq = line.chars().counts_by(|c| c);
        if freq.values().any(|n| *n == 2) {
            has2 += 1;
        }
        if freq.values().any(|n| *n == 3) {
            has3 += 1;
        }
    }

    println!("{} part 1: {}", title, has2 * has3);

    for a in input.lines() {
        for b in input.lines() {
            if a.chars().zip(b.chars()).filter(|(a, b)| *a != *b).count() == 1 {
                println!("{}", a);
                println!("{}", b);
            }
        }
    }
}

const INPUT_DEMO: &str = "";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
