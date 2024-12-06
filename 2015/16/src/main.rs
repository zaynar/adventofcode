use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data = input
        .lines()
        .map(|line| {
            let (name, items) = line.split_once(": ").unwrap();
            (name, items.split(", ").map(|item| {
                let (thing, count) = item.split_once(": ").unwrap();
                let count: i32 = count.parse().unwrap();
                (thing, count)
            }).collect_vec())
        })
        .collect_vec();

    println!("{:?}", data);

    let reqs = HashMap::from([
    ("children", 3),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1)
    ]);

    println!("{} part 1: {:?}", title, data.iter().filter(|(name, items)| items.iter().all(|(t,c)| reqs[t] == *c) ).collect_vec());

    println!("{} part 2: {:?}", title, data.iter().filter(|(name, items)| items.iter().all(|(t,c)|
        match *t {
            "cats" | "trees" => *c > reqs[t],
            "pomeranians" | "goldfish" => *c < reqs[t],
            _ => reqs[t] == *c
        }
    ) ).collect_vec());
}

fn main() {
    run("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
