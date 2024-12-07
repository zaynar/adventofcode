use std::collections::HashSet;

use fancy_regex::{Captures, Regex};
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let re = Regex::new(r"(.)(?!\1)(.)\2\1").unwrap();
    let re2 = Regex::new(r"\[[^\]]*(.)(?!\1)(.)\2\1").unwrap();
    let hyper = Regex::new(r"(?x) \[ ( [^\]]* ) \]").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        if re.is_match(line).unwrap() && !re2.is_match(line).unwrap() {
            part1 += 1;
            // println!("{}", line)
        }

        let mut hyper = false;
        let mut pat0 = HashSet::new();
        let mut pat1 = HashSet::new();
        for (a, b, c) in line.chars().tuple_windows() {
            if a == '[' {
                hyper = true;
            } else if a == ']' {
                hyper = false;
            } else if a != b && a == c {
                if hyper {
                    pat1.insert((b, a));
                } else {
                    pat0.insert((a, b));
                }
            }
        }
        if pat0.intersection(&pat1).count() > 0 {
            part2 += 1;
            // println!("TLS {}", line)
        }
    }
    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn

aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
