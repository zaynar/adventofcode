// Part 1: 60 mins
// Part 1+2: 165 mins

use std::collections::{HashMap, HashSet};

fn pad_paths(pad: &HashMap<char, (i32, i32)>, from: char, to: char) -> HashSet<String> {
    let dir = HashMap::from([
        ((-1, 0), '<'),
        ((1, 0), '>'),
        ((0, -1), '^'),
        ((0, 1), 'v'),
    ]);

    let mut ret = HashSet::new();

    let mut open = vec![(from, "".to_owned())];
    while let Some((cur, path)) = open.pop() {

        if cur == to {
            ret.insert(path);
            continue;
        }

        for other in pad.keys() {
            if pad[&to].0.abs_diff(pad[other].0) < pad[&to].0.abs_diff(pad[&cur].0) ||
                pad[&to].1.abs_diff(pad[other].1) < pad[&to].1.abs_diff(pad[&cur].1) {
                if let Some(d) = dir.get(&(pad[other].0 - pad[&cur].0, pad[other].1 - pad[&cur].1)) {
                    let mut p2 = path.clone();
                    p2.push(*d);
                    open.push((*other, p2));
                }
            }
        }

    }

    ret
}

fn numpad_paths(from: char, to: char) -> HashSet<String> {
    let pad: HashMap<char, (i32, i32)> = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    pad_paths(&pad, from, to)
}

fn dirpad_paths(from: char, to: char) -> HashSet<String> {
    let pad: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    pad_paths(&pad, from, to)
}

fn expand(paths: &[HashSet<String>], v: String) -> HashSet<String> {
    if paths.is_empty() {
        return HashSet::from([v]);
    }

    let mut r = HashSet::new();
    for p in &paths[0] {
        let mut v2 = v.clone();
        v2.push_str(&p);
        r = HashSet::from_iter(r.union(&expand(&paths[1..], v2)).cloned());
    }

    r
}

/*

<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
  v <<   A >>  ^ A   <   A > A  v  A   <  ^ AA > A   < v  AAA >  ^ A
         <       A       ^   A     >        ^^   A        vvv      A
                 0           2                   9                 A

So each segment (a string ending in A, implicitly starting with all robots at A)
goes [A]xyzA -> [A]mnopqrstA, and is independent of other segments.

=> Each segment maps onto multiple segments, in several different ways
(different routes through the dirpad), so we can recurse through the segments
and pick the lowest-cost way

*/

fn seg_cost(p: &String, reps: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    assert_eq!(p.chars().last(), Some('A'));

    if reps == 0 {
        return p.len();
    }

    if let Some(r) = cache.get(&(p.clone(), reps)) {
        return *r;
    }

    let mut cost = 0;
    let mut pos1 = 'A';

    for c in p.chars() {
        cost += dirpad_paths(pos1, c).iter().map(|s|
            seg_cost(&(s.clone() + "A"), reps - 1, cache)
        ).min().unwrap();
        pos1 = c;
    }

    cache.insert((p.clone(), reps), cost);

    cost
}


fn run(title: &str, input: &str, reps: usize) {
    let mut part2 = 0;

    for line in input.lines() {

        let mut paths0 = Vec::new();

        let mut pos0 = 'A';
        for c in line.chars() {
            paths0.push(numpad_paths(pos0, c));
            paths0.push(HashSet::from(["A".to_owned()]));
            pos0 = c;
        }

        let paths1 = expand(&paths0, "".to_owned());

        println!("{}:", line);

        let mut cache = HashMap::new();
        let mut best_cost = usize::MAX;
        for p1 in paths1 {
            let cost = seg_cost(&p1, reps, &mut cache);
            println!("  {} = {}", p1, cost);
            best_cost = best_cost.min(cost);
        }

        part2 += best_cost * usize::from_str_radix(&line[0..3], 10).unwrap();

    }

    println!("# {} result: {}", title, part2);
}


const INPUT_DEMO: &str = "029A
980A
179A
456A
379A
";

fn main() {
    run("demo", INPUT_DEMO, 2);
    run("input p1", &std::fs::read_to_string("21/input.txt").unwrap(), 2);
    run("input p2", &std::fs::read_to_string("21/input.txt").unwrap(), 25);
}
