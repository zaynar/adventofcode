// Part 1: 8 mins
// Part 1+2: 20 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (rules, rest) = input.split_once("\n\nyour ticket:\n").unwrap();
    let (your, nearby) = rest.split_once("\n\nnearby tickets:\n").unwrap();

    let rules: HashMap<String, (u32, u32, u32, u32)> = rules.lines().map(|r| {
        let (name, rest) = r.split_once(": ").unwrap();
        let (range0, range1) = rest.split_once(" or ").unwrap();
        let (a, b) = range0.split_once("-").unwrap();
        let (c, d) = range1.split_once("-").unwrap();
        (name.to_owned(), (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap(), d.parse().unwrap()))
    }).collect();

    let your: Vec<u32> = your.split(",").map(|n| n.parse().unwrap()).collect();

    let nearby: Vec<Vec<u32>> = nearby.lines().map(|n| n.split(",").map(|n| n.parse().unwrap()).collect()).collect();

    // println!("{:?}", rules);
    // println!("{:?}", your);
    // println!("{:?}", nearby);

    let mut tickets = vec![];

    let mut part1 = 0;
    for ticket in &nearby {
        let mut tok = true;
        for v in ticket {
            let mut ok = false;
            for (name, (a, b, c, d)) in &rules {
                if (a <= v && v <= b) || (c <= v && v <= d) {
                    ok = true;
                }
            }
            if !ok {
                part1 += v;
                tok = false;
            }
        }
        if tok {
            tickets.push(ticket.clone());
        }
    }

    println!("{} part 1: {}", title, part1);

    let mut maybe: Vec<HashSet<String>> = your.iter().map(|_| HashSet::from_iter(rules.keys().cloned())).collect();

    // println!("{:#?}", maybe);

    for ticket in &tickets {
        for (i, v) in ticket.iter().enumerate() {
            for (name, (a, b, c, d)) in &rules {
                if !((a <= v && v <= b) || (c <= v && v <= d)) {
                    maybe[i].remove(name);
                }
            }
        }
    }

    let mut definite = HashMap::new();
    while definite.len() != your.len() {

        for (i, v) in maybe.iter().enumerate() {
            if v.len() == 1 {
                definite.insert(v.iter().next().unwrap().clone(), i);
            }
        }

        for m in &mut maybe {
            for d in definite.keys() {
                m.remove(d);
            }
        }

    }

    println!("{:#?}", definite);

    println!("{} part 2: {}", title, definite.iter().filter(|d| d.0.starts_with("departure")).map(|d| your[*d.1] as u64).product::<u64>());
}

const INPUT_DEMO: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
