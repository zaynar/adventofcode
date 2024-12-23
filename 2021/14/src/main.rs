// Part 1: 10 mins
// Part 1+2: 11 mins

use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();

    for rule in rules.lines() {
        map.insert(
            (rule.chars().nth(0).unwrap(), rule.chars().nth(1).unwrap()),
            rule.chars().nth(6).unwrap(),
        );
    }

    let mut counts = HashMap::new();
    for (a, b) in template.chars().tuple_windows() {
        *counts.entry((a, b)).or_insert(0) += 1;
    }

    println!("{:?}", counts);

    for i in 0..40 {
        let mut new = HashMap::new();
        for ((a, b), n) in &counts {
            let c = map.get(&(*a, *b)).unwrap();
            *new.entry((*a, *c)).or_insert(0_i64) += n;
            *new.entry((*c, *b)).or_insert(0_i64) += n;
        }
        counts = new;
    }

    // println!("{:?}", counts);

    let mut e = HashMap::new();
    for ((a, b), n) in &counts {
        *e.entry(*a).or_insert(0) += n;
    }
    *e.entry(template.chars().last().unwrap()).or_insert(0) += 1;
    // println!("{:#?}", e);

    println!("{} part N: {}", title, e.values().max().unwrap() - e.values().min().unwrap());

    // println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("14/input.txt").unwrap());
}
