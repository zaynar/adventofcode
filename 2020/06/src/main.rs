// Part 1: 2 mins
// Part 1+2: 6 mins

use std::collections::HashSet;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut part1 = 0;
    for group in input.split("\n\n") {
        let mut s = group.chars().counts();
        s.remove(&'\n');
        part1 += s.len();
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;
    for group in input.split("\n\n") {
        let mut s: HashSet<char> = HashSet::from_iter('a'..='z');
        for line in group.trim().lines() {
            s = HashSet::from_iter(s.intersection(&HashSet::from_iter(line.chars())).cloned());
        }
        part2 += s.len();
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "abc

a
b
c

ab
ac

a
a
a
a

b
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
