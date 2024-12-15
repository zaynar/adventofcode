// Part 1: 4 mins
// Part 1+2: 6 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {

    let mut edges = HashMap::new();
    for line in input.lines() {
        let (src, dsts) = line.split_once(" <-> ").unwrap();
        let mut dsts = dsts.split(", ").collect_vec();
        edges.entry(src).or_insert_with(|| Vec::new()).append(&mut dsts);
    }

    // println!("{:?}", edges);

    let mut open = Vec::new();
    let mut seen = HashSet::new();

    open.push("0");

    while let Some(n) = open.pop() {
        if !seen.insert(n) {
            continue;
        }

        for d in &edges[n] {
            open.push(d);
        }
    }

    println!("{} part 1: {}", title, seen.len());

    let mut groups = 0;
    let mut open = Vec::new();
    let mut seen = HashSet::new();

    while let Some(root) = edges.keys().find(|k| !seen.contains(k))
    {
        groups += 1;
        open.push(root);

        while let Some(n) = open.pop() {
            if !seen.insert(n) {
                continue;
            }

            for d in &edges[n] {
                open.push(d);
            }
        }
    }

    println!("{} part 2: {}", title, groups);
}

const INPUT_DEMO: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
