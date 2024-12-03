use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<(&str, &str)> = input
        .lines()
        .map(|line| {
            line.split(")").next_tuple().unwrap()
        })
        .collect();

    // println!("{:?}", data);

    let mut parents = HashMap::new();
    let mut edges = HashMap::new();
    for (parent, child) in &data {
        parents.insert(*child, *parent);
        edges.entry(*child).or_insert_with(|| Vec::new()).push(*parent);
        edges.entry(*parent).or_insert_with(|| Vec::new()).push(*child);
    }

    let mut num_direct = 0;
    let mut num_indirect = 0;
    for src in parents.keys() {
        num_direct += 1;

        let mut p = parents[src];
        while let Some(n) = parents.get(p) {
            num_indirect += 1;
            p = n;
        }
    }

    // println!("{:?}", edges);

    let mut dists: HashMap<&str, usize> = HashMap::new();
    dists.insert("YOU", 0);
    let mut open = VecDeque::new();
    open.push_back("YOU");
    while let Some(p) = open.pop_front() {

        let dist = dists[p];

        for n in edges.get(p).unwrap() {
            if dists.get(n).is_none() {
                dists.insert(n, dist + 1);
                open.push_back(n);
            }
        }
    }

    // println!("{:?}", dists);

    println!("{} part 1: {}", title, num_direct + num_indirect);

    println!("{} part 2: {}", title, dists["SAN"] - 2);
}

const INPUT_DEMO: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
