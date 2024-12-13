// Part 1: 8 mins
// Part 1+2: 16 mins

use std::collections::HashSet;

use itertools::Itertools;

fn strongest(port: u32, left: &HashSet<(u32, u32)>) -> u32 {
    left.iter().filter_map(|c| {
        if c.0 == port || c.1 == port {
            let mut next = left.clone();
            next.remove(&c);
            let p2 = if c.0 == port { c.1 } else { c.0 };
            Some(c.0 + c.1 + strongest(p2, &next))
        } else {
            None
        }
    }).max().unwrap_or(0)
}

fn paths(port: u32, left: &HashSet<(u32, u32)>, len: u32, strength: u32, out: &mut HashSet<(u32, u32)>) {
    out.insert((len, strength));
    left.iter().for_each(|c| {
        if c.0 == port || c.1 == port {
            let mut next = left.clone();
            next.remove(&c);
            let p2 = if c.0 == port { c.1 } else { c.0 };
            paths(p2, &next, len + 1, strength + c.0 + c.1, out);
        }
    });
}

fn run(title: &str, input: &str) {
    let data: HashSet<(u32, u32)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("/").unwrap();
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();
            (a.min(b), a.max(b))
        }).collect();

    println!("{} part 1: {}", title, strongest(0, &data));

    let mut out = HashSet::new();
    paths(0, &data, 0, 0, &mut out);
    let out = out.iter().sorted_by_key(|(l, s)| -(*l as i32)).collect_vec();
    let len = out.iter().map(|(l, s)| l).max().unwrap();
    let out = out.iter().filter(|(l, s)| l == len).sorted().last().unwrap();
    println!("{} part 2: {:?}", title, out);
}

const INPUT_DEMO: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("24/input.txt").unwrap());
}
