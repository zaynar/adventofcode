// Part 1: 10 mins
// Part 1+2: 43 mins

use std::collections::HashMap;

use aocpath::Pathfinder;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (src, dst) = line.split_once(": ").unwrap();
        let dst = dst
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect_vec();
        data.insert(src.to_owned(), dst);
    }

    // println!("{data:?}");

    struct PathContext {
        data: HashMap<String, Vec<String>>,
        count: usize,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
    struct Node {
        label: String,
        history: Vec<String>,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut hist = node.history.clone();
            hist.push(node.label.clone());
            self.data
                .get(&node.label)
                .unwrap()
                .iter()
                .filter_map(|s| {
                    if hist.contains(&s) {
                        None
                    } else {
                        Some((
                            1,
                            Node {
                                label: s.clone(),
                                history: hist.clone(),
                            },
                        ))
                    }
                })
                .collect()
        }

        fn found_path(&mut self, id: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            if id.label == "out" {
                self.count += 1;
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    let mut ctx = PathContext { data, count: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.dijkstra_all(
        &mut ctx,
        Node {
            label: "you".to_owned(),
            history: vec![],
        },
    );

    println!("{} part 1: {}", title, ctx.count);
}

fn run2(title: &str, input: &str) {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (src, dst) = line.split_once(": ").unwrap();
        let dst = dst
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect_vec();
        data.insert(src.to_owned(), dst);
    }

    // println!("digraph G {{");
    // for (k, v) in &data {
    //     for d in v {
    //         println!("{k} -> {d};");
    //     }
    // }
    // println!("}}");

    let mut sorted: Vec<String> = vec![];

    let mut incoming: HashMap<String, usize> = HashMap::new();
    for (src, dsts) in &data {
        incoming.insert(src.clone(), 0);
    }
    for (src, dsts) in &data {
        for dst in dsts {
            *incoming.entry(dst.clone()).or_default() += 1;
        }
    }

    println!("{incoming:?}");

    // Topological sort
    loop {
        let mut dirty = false;

        if let Some((n, e)) = incoming
            .iter()
            .find(|(n, e)| **e == 0 && !sorted.contains(n))
        {
            sorted.push(n.clone());

            if let Some(dsts) = data.get(n) {
                for dst in dsts {
                    *incoming.get_mut(dst).unwrap() -= 1;
                    dirty = true;
                }
            }
        }

        if !dirty {
            break;
        }
    }

    println!("{sorted:?}");

    let mut paths: HashMap<(&str, &str), usize> = HashMap::new();

    // Count number of paths from origin to each node
    // (propagating in topological order)
    for origin in ["svr", "fft", "dac"] {
        paths.insert((origin, origin), 1);

        for node in &sorted {

            let p = paths.get(&(origin, node)).copied().unwrap_or(0);

            if let Some(dsts) = data.get(node) {
                for dst in dsts {
                    *paths.entry((origin, dst)).or_default() += p;
                }
            }
        }
    }

    for origin in ["svr", "fft", "dac"] {
        // println!("{:?}", paths.iter().filter(|(a, b)| a.0 == origin).sorted().collect_vec());
    }

    println!("{} part 2: {}", title, paths[&("svr", "fft")] * paths[&("fft", "dac")] * paths[&("dac", "out")]);
    println!("{} part 2: {}", title, paths[&("svr", "dac")] * paths[&("dac", "fft")] * paths[&("fft", "out")]);
}

const INPUT_DEMO: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const INPUT_DEMO2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("11/input.txt").unwrap());
    run2("demo", INPUT_DEMO2);
    run2("input", &std::fs::read_to_string("11/input.txt").unwrap());
}
