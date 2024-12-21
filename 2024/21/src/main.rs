// Part 1: 60 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn numpad_paths(from: char, to: char) -> HashSet<Vec<char>> {
    let numpad: HashMap<char, (i32, i32)> = HashMap::from([
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

    let dir = HashMap::from([
        ((-1, 0), '<'),
        ((1, 0), '>'),
        ((0, -1), '^'),
        ((0, 1), 'v'),
    ]);

    let mut ret = HashSet::new();

    let mut open = vec![(from, vec![])];
    while let Some((cur, path)) = open.pop() {

        if cur == to {
            ret.insert(path);
            continue;
        }

        for other in numpad.keys() {
            if numpad[&to].0.abs_diff(numpad[other].0) < numpad[&to].0.abs_diff(numpad[&cur].0) ||
                numpad[&to].1.abs_diff(numpad[other].1) < numpad[&to].1.abs_diff(numpad[&cur].1) {
                if let Some(d) = dir.get(&(numpad[other].0 - numpad[&cur].0, numpad[other].1 - numpad[&cur].1))
                {
                    let mut p2 = path.clone();
                    p2.push(*d);
                    open.push((*other, p2));
                }
            }
        }

    }


    // let mut dx = numpad[&to].0 - numpad[&from].0;
    // let mut dy = numpad[&to].1 - numpad[&from].1;

    // let mut r = Vec::new();
    // while dy < 0 {
    //     r.push('^');
    //     dy += 1;
    // }
    // while dy > 0 {
    //     r.push('v');
    //     dy -= 1;
    // }
    // while dx < 0 {
    //     r.push('<');
    //     dx += 1;
    // }
    // while dx > 0 {
    //     r.push('>');
    //     dx -= 1;
    // }

    // let mut ret = HashSet::new();

    // for p in r.iter().copied().permutations(r.len()) {
    //     ret.insert(p);
    // }

    ret
}

fn dir_paths(from: char, to: char) -> HashSet<Vec<char>> {
    let dirpad = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut ret = HashSet::new();

    {
        let mut dx = dirpad[&to].0 - dirpad[&from].0;
        let mut dy = dirpad[&to].1 - dirpad[&from].1;

        let mut r = Vec::new();
        while dy < 0 {
            r.push('^');
            dy += 1;
        }
        while dy > 0 {
            r.push('v');
            dy -= 1;
        }
        while dx < 0 {
            r.push('<');
            dx += 1;
        }
        while dx > 0 {
            r.push('>');
            dx -= 1;
        }
        ret.insert(r);
    }

    if dirpad[&from].1 != 3 && dirpad[&to].1 != 1 {
        let mut dx = dirpad[&to].0 - dirpad[&from].0;
        let mut dy = dirpad[&to].1 - dirpad[&from].1;

        let mut r = Vec::new();
        while dx < 0 {
            r.push('<');
            dx += 1;
        }
        while dx > 0 {
            r.push('>');
            dx -= 1;
        }
        while dy < 0 {
            r.push('^');
            dy += 1;
        }
        while dy > 0 {
            r.push('v');
            dy -= 1;
        }
        ret.insert(r);
    }

    ret
}

fn expand(paths: &[HashSet<Vec<char>>], pos: char, v: Vec<char>) -> HashSet<Vec<char>> {
    if paths.is_empty() {
        // println!("- {}", v.iter().collect::<String>());
        return HashSet::from([v]);
    }

    let mut r = HashSet::new();
    for p in &paths[0] {
        let mut v2 = v.clone();
        v2.append(&mut p.clone());
        r = HashSet::from_iter(r.union(&expand(&paths[1..], pos, v2)).cloned());
    }

    r
}

fn run(title: &str, input: &str) {

    let mut part1 = 0;

    for line in input.lines() {

        println!("# {}", line);

        let mut fin = Vec::new();

        let mut paths0 = Vec::new();

        let mut pos0 = 'A';
        for c in line.chars() {
            paths0.push(numpad_paths(pos0, c));
            paths0.push(HashSet::from([vec!['A']]));
            pos0 = c;
        }
        println!("{:?}", paths0);

        let paths1 = expand(&paths0, 'A', vec![]);
        for p1 in paths1 {
            println!("  {}", p1.iter().collect::<String>());

            let mut pos1 = 'A';
            let mut paths2 = Vec::new();
            for c in p1 {
                paths2.push(dir_paths(pos1, c));
                paths2.push(HashSet::from([vec!['A']]));
                pos1 = c;
            }

            let paths3 = expand(&paths2, 'A', vec![]);
            for p3 in paths3 {
                println!("    {}", p3.iter().collect::<String>());

                let mut pos2 = 'A';
                let mut paths4 = Vec::new();
                for c in p3 {
                    paths4.push(dir_paths(pos2, c));
                    paths4.push(HashSet::from([vec!['A']]));
                    pos2 = c;
                }

                let paths5 = expand(&paths4, 'A', vec![]);
                for p5 in paths5 {
                    println!("      {}", p5.iter().collect::<String>());
                    fin.push(p5.iter().collect::<String>());
                }
            }
        }

        let min = fin.iter().map(|n| n.len()).min().unwrap();
        println!("{}", min);
        part1 += min * usize::from_str_radix(&line[0..3], 10).unwrap();

        println!("====");
    }

    // 171230 too high

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "029A
980A
179A
456A
379A
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("21/input.txt").unwrap());
    run("demo", "459A");
}
