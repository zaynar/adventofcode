use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {

    let mut edges = HashMap::new();

    for line in input.lines() {
        let cs = line.split_whitespace().collect_vec();
        let (from, to, dist) = (cs[0], cs[2], str::parse::<u32>(cs[4]).unwrap());

        edges.entry(from).or_insert_with(|| HashMap::new()).insert(to, dist);
        edges.entry(to).or_insert_with(|| HashMap::new()).insert(from, dist);
    }

    // println!("{:?}", edges);

    let mut shortest = u32::MAX;
    let mut longest = 0;
    for route in edges.keys().permutations(edges.len()) {
        let dist: u32 = route.windows(2).map(|w| edges[w[0]][w[1]]).sum();
        // println!("{:?} = {}", route, dist);
        shortest = shortest.min(dist);
        longest = longest.max(dist);
    }

    println!("{} part 1: {}", title, shortest);
    println!("{} part 2: {}", title, longest);
}

const INPUT_DEMO: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
