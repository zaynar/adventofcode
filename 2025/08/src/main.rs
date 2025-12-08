// Part 1: 25 mins
// Part 1+2: 32 mins

extern crate nalgebra as na;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use na::Vector3;

fn run(title: &str, input: &str, limit: usize) {
    let data: Vec<_> = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(",")
                .map(|n| str::parse::<i64>(n).unwrap())
                .collect_tuple()
                .unwrap();
            Vector3::new(x, y, z)
        })
        .collect();

    let mut pairs = vec![];
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let d = data[i] - data[j];
            let dist = d.dot(&d);
            pairs.push((dist, i, j));
        }
    }
    pairs.sort();

    if false {
        let mut nodes = HashSet::new();
        let mut edges: HashMap<usize, HashSet<usize>> = HashMap::new();

        for i in 0..limit {
            let (dist, i, j) = pairs[i];
            nodes.insert(i);
            nodes.insert(j);
            edges.entry(i).or_default().insert(j);
            edges.entry(j).or_default().insert(i);
        }

        let mut circuits = vec![];
        let mut assigned: HashMap<usize, usize> = HashMap::new();

        for &start in &nodes {
            if assigned.contains_key(&start) {
                continue;
            }

            let c = circuits.len();
            let mut added = vec![];

            let mut open = vec![start];
            while let Some(node) = open.pop() {
                if assigned.contains_key(&node) {
                    continue;
                }
                assigned.insert(node, c);
                added.push(node);

                for &n in &edges[&node] {
                    open.push(n);
                }
            }

            circuits.push(added);
        }

        // println!("{circuits:?}");

        let part1: usize = circuits
            .iter()
            .map(|c| c.len())
            .sorted()
            .rev()
            .take(3)
            .product();

        println!("{} part 1: {}", title, part1);
    }

    {
        for limit in 1.. {
            let mut nodes = HashSet::new();
            let mut edges: HashMap<usize, HashSet<usize>> = HashMap::new();

            for &(dist, i, j) in &pairs {
                nodes.insert(i);
                nodes.insert(j);
            }

            for &(dist, i, j) in pairs.iter().take(limit) {
                edges.entry(i).or_default().insert(j);
                edges.entry(j).or_default().insert(i);
            }

            let mut circuits = vec![];
            let mut assigned: HashMap<usize, usize> = HashMap::new();

            for &start in &nodes {
                if assigned.contains_key(&start) {
                    continue;
                }

                let c = circuits.len();
                let mut added = vec![];

                let mut open = vec![start];
                while let Some(node) = open.pop() {
                    if assigned.contains_key(&node) {
                        continue;
                    }
                    assigned.insert(node, c);
                    added.push(node);

                    if edges.contains_key(&node) {
                        for &n in &edges[&node] {
                            open.push(n);
                        }
                    }
                }

                circuits.push(added);
            }

            let (_, n0, n1) = pairs[limit - 1];
            let part2 = data[n0].x * data[n1].x;
            // println!("{limit} {circuits:?}");
            println!("{limit} {} {}", circuits.len(), pairs[limit].0);

            if circuits.len() == 1 {
                println!("{} part 2: {}", title, part2);
                break;
            }
        }
    }
}

const INPUT_DEMO: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() {
    run("demo", INPUT_DEMO, 10);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap(), 1000);
}
