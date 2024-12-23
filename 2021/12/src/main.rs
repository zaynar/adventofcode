// Part 1: 7 mins
// Part 1+2: 12 mins

use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Path {
    node: String,
    path: Vec<String>,
    duped: bool,
}

fn run(title: &str, input: &str) {

    let mut edges = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        edges.entry(a.to_owned()).or_insert_with(|| HashSet::new()).insert(b.to_owned());
        edges.entry(b.to_owned()).or_insert_with(|| HashSet::new()).insert(a.to_owned());
    }

    let mut part1 = 0;
    let mut paths = vec![Path { node: "start".to_owned(), path: vec!["start".to_owned()], duped: false }];
    let mut seen = HashSet::new();
    while let Some(p) = paths.pop() {
        if !seen.insert(p.clone()) {
            continue;
        }

        if p.node == "end" {
            part1 += 1;
            continue;
        }

        for n in edges.get(&p.node).unwrap() {
            if *n == n.to_ascii_lowercase() && p.path.contains(n) {
                continue;
            }

            let mut np = Path { node: n.clone(), path: p.path.clone(), duped: false };
            np.path.push(n.clone());
            paths.push(np);
        }
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;
    let mut paths = vec![Path { node: "start".to_owned(), path: vec!["start".to_owned()], duped: false }];
    let mut seen = HashSet::new();
    while let Some(p) = paths.pop() {
        if !seen.insert(p.clone()) {
            continue;
        }

        if p.node == "end" {
            // println!("{:?}", p);
            part2 += 1;
            continue;
        }

        for n in edges.get(&p.node).unwrap() {
            if *n == "start" { continue; }
            if p.duped && *n == n.to_ascii_lowercase() && p.path.contains(n) {
                continue;
            }

            let mut np = Path { node: n.clone(), path: p.path.clone(), duped: p.duped || (*n == n.to_ascii_lowercase() && p.path.contains(n)) };
            np.path.push(n.clone());
            paths.push(np);
        }
    }


    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
