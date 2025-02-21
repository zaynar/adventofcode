use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let edges = std::fs::read_to_string("input").unwrap().lines().flat_map(|line| {
        let (a, b) = line.split_once(": ").unwrap();
        let bs = b.split_ascii_whitespace();
        bs.map(|b| [ (a.to_owned(), b.to_owned()), (b.to_owned(), a.to_owned()) ]).flatten()
    }).collect_vec();
    // println!("{:?}", edges);
    // println!("graph G {{");
    // for edge in edges {
    //     if edge.0 < edge.1 {
    //         println!("{} -- {}", edge.0, edge.1);
    //     }
    // }
    // println!("}}");

    // Render with neato to determine the edges to cut

    let cuts = [
        ("vps", "pzc"),
        ("dph", "cvx"),
        ("xvk", "sgc"),
    ];
    let edges = edges.iter().filter(|(a, b)| !cuts.contains(&(a, b)) && !cuts.contains(&(b, a))).collect_vec();

    let mut edge_map: HashMap<String, Vec<String>> = HashMap::new();
    for edge in edges.iter() {
        edge_map.entry(edge.0.clone()).or_default().push(edge.1.clone());
    }

    let mut answer = Vec::new();
    for start in ["vps", "pzc"] {
        let mut visited = HashSet::new();
        let mut open = Vec::new();
        open.push(start);
        while let Some(n) = open.pop() {
            if visited.insert(n) {
                for p in &edge_map[n] {
                    open.push(p);
                }
            }
        }
        answer.push(visited.len());
    }
    println!("{} - {:?}", answer.iter().product::<usize>(), answer);
}
