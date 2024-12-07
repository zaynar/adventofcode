use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut counts = Vec::new();
    for i in 0..8 {
        counts.push(HashMap::new());
    }

    for line in input.lines() {
        for i in 0..line.len() {
            *counts[i].entry(line.as_bytes()[i]).or_insert(0) += 1;
        }
    }

    for i in 0..8 {
        if let Some(c) = counts[i].iter().sorted_by_key(|(&k, &v)| -v).next() {
            print!("{}", *c.0 as char);
        }
    }
    println!();

    for i in 0..8 {
        if let Some(c) = counts[i].iter().sorted_by_key(|(&k, &v)| v).next() {
            print!("{}", *c.0 as char);
        }
    }
    println!();
}

const INPUT_DEMO: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
