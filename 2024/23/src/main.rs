// Part 1: 11 mins
// Part 2: 24 mins

use std::collections::{HashMap, HashSet};

fn run(title: &str, input: &str) {
    let mut links: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (src, dst) = (line[0..2].to_owned(), line[3..5].to_owned());
        links.entry(src.clone()).or_insert_with(|| HashSet::new()).insert(dst.clone());
        links.entry(dst.clone()).or_insert_with(|| HashSet::new()).insert(src.clone());
    }

    // println!("{:?}", links);

    let mut groups = HashSet::new();

    'OUTER: for a in links.keys() {
        for b in &links[a] {
            for c in &links[b] {
                if links[c].contains(a) {
                    let mut g = vec![a.to_owned(), b.to_owned(), c.to_owned()];
                    g.sort();
                    groups.insert(g);
                }
            }
        }
    }

    // println!("{:?}", groups);

    let part1 = groups.iter().filter(|v| v.iter().any(|s| s.starts_with("t"))).count();

    println!("{} part 1: {}", title, part1);
}

fn run2(title: &str, input: &str) {
    let mut links: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (src, dst) = (line[0..2].to_owned(), line[3..5].to_owned());
        links.entry(src.clone()).or_insert_with(|| HashSet::new()).insert(dst.clone());
        links.entry(dst.clone()).or_insert_with(|| HashSet::new()).insert(src.clone());
    }

    // println!("{:?}", links);

    let mut groups = HashSet::new();

    'OUTER: for a in links.keys() {

        let mut group = vec![a.clone()];
        loop {
            // println!("> {:?}", group);
            let prev = group.len();

            let mut mutual = links.get(&group[0]).unwrap().clone();
            for i in 1..group.len() {
                let t = mutual.intersection(&links.get(&group[i]).unwrap()).cloned();
                mutual = HashSet::from_iter(t);
            }

            if let Some(m) = mutual.iter().next() {
                group.push(m.clone());
            }

            if group.len() == prev {
                break;
            }
        }

        group.sort();
        groups.insert(group.clone());
        // println!("{:?}", group);

        // break;
    }

    // println!("{:?}", groups);

    let largest = groups.iter().map(|n| n.len()).max().unwrap();
    println!("{} part 2: {:?}", title, groups.iter().filter(|n| n.len() == largest).map(|n| n.join(",")).collect::<Vec<_>>());
}

const INPUT_DEMO: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("23/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("23/input.txt").unwrap());
}
