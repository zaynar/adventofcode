// Part 1: 9 mins
// Part 1+2: 20 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut preds: HashMap<char, Vec<char>> = HashMap::new();
    for line in input.lines() {
        preds.entry(line.chars().nth(5).unwrap()).or_insert(Vec::new());
        preds.entry(line.chars().nth(36).unwrap()).or_insert(Vec::new()).push(line.chars().nth(5).unwrap());
    }

    // println!("{:?}", preds);

    print!("{} part 1: ", title);

    let mut seen: HashSet<char> = HashSet::new();
    'O: loop {
        match preds.iter().filter(|(k, v)| !seen.contains(k) && v.iter().all(|p| seen.contains(&p))).sorted().next() {
            Some(n) => {
                print!("{}", n.0);
                seen.insert(*n.0);
            }
            None => { break 'O; }
        }
    }
    println!();
}

fn run2(title: &str, input: &str, workers: usize, base_time: u32) {
    let mut preds: HashMap<char, Vec<char>> = HashMap::new();
    for line in input.lines() {
        preds.entry(line.chars().nth(5).unwrap()).or_insert(Vec::new());
        preds.entry(line.chars().nth(36).unwrap()).or_insert(Vec::new()).push(line.chars().nth(5).unwrap());
    }

    // println!("{:?}", preds);

    let mut out = Vec::new();

    let mut ws: Vec<(Option<char>, u32)>= vec![(None, 0); workers];

    let mut seen: HashSet<char> = HashSet::new();
    let mut busy: HashSet<char> = HashSet::new();
    for t in 0.. {
        for w in &mut ws {
            *w = match *w {
                (Some(c), n) if n > 1 => {
                    (Some(c), n - 1)
                }
                (Some(c), _) => {
                    out.push(c);
                    seen.insert(c);
                    (None, 0)
                }
                (None, n) => (None, n)
            };
        }

        let mut ready = preds.iter().filter(|(k, v)| !busy.contains(k) && v.iter().all(|p| seen.contains(&p))).sorted();

        for w in &mut ws {
            if w.0.is_none() {
                match ready.next() {
                    Some(n) => {
                        *w = (Some(*n.0), base_time + (1 + *n.0 as u8 - 'A' as u8) as u32);
                        busy.insert(*n.0);
                    }
                    None => ()
                }
            }
        }

        // println!("{} {:?}", t, ws);

        if ws.iter().all(|w| w.0.is_none()) {
            // println!("{}", out.iter().collect::<String>());
            println!("{} part 2: {}", title, t);
            break;
        }
    }
}

const INPUT_DEMO: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

fn main() {
    run("demo", INPUT_DEMO);
    run2("demo", INPUT_DEMO, 2, 0);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("07/input.txt").unwrap(), 5, 60);
}
