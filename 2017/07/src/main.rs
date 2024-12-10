// Part 1: 9 mins
// Part 1+1: 22 mins

use std::collections::HashMap;

use itertools::Itertools;

fn recurse(n: &String, weights: &HashMap<String, i32>, edges: &HashMap<String, Vec<String>>) -> i32 {
    let cw = edges.get(n).unwrap_or(&Vec::new()).iter().map(|c| {
        recurse(c, weights, edges)
    }).collect_vec();
    if cw.iter().any(|c| *c != cw[0]) {
        println!("{} {:?} {:?}", n, cw, edges.get(n));
        for z in edges.get(n).unwrap() {
            println!(" {}={}", z, weights[z]);
        }
        // panic!();
    }

    weights[n] + cw.iter().sum::<i32>()
}

fn run(title: &str, input: &str) {
    let mut edges = HashMap::new();
    let mut revedges = HashMap::new();
    let mut weights = HashMap::new();
    let data: Vec<(String, i32)> = input
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(" ").unwrap();
            let weight = if let Some((weight, rest)) = rest.split_once(" -> ") {
                for dst in rest.split(", ") {
                    edges.entry(name.to_owned()).or_insert_with(|| Vec::new()).push(dst.to_owned());
                    revedges.entry(dst.to_owned()).or_insert_with(|| Vec::new()).push(name.to_owned());
                }
                weight
            } else {
                rest
            };
            // println!("{}#{}", name, weight);
            let weight = weight.strip_prefix("(").unwrap().strip_suffix(")").unwrap().parse().unwrap();

            weights.insert(name.to_owned(), weight);
            (name.to_owned(), weight)
        })
        .collect();

    println!("{:?}", data);
    println!("{:?}", edges);
    println!("{:?}", revedges);

    let root = data.iter().filter_map(|(n, w)| if revedges.contains_key(n) { None } else { Some(n) }).collect_vec();
    println!("{} part 1: {:?}", title, root);


    recurse(root[0], &weights, &edges);
}

const INPUT_DEMO: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
