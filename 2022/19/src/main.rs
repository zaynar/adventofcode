// Part 1: 22 mins
// Part 1+2: 35 mins

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Node {
    // ore, clay, obsidian, geode
    resources: [u16; 4],
    bots: [u16; 4],
    time: u16,
}

fn run(title: &str, input: &str) {
    let blueprints: Vec<Vec<u16>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|n| n.parse::<u16>().ok())
                .collect()
        })
        .collect();

    println!("{:?}", blueprints);

    let mut part1 = 0;

    for (i, bp) in blueprints.iter().enumerate() {
        let mut open = VecDeque::from([
            Node {
                resources: [0, 0, 0, 0],
                bots: [1, 0, 0, 0],
                time: 0,
            }
        ]);

        let mut visited = HashSet::new();

        let (oo, co, bo, bc, go, gb) = bp.iter().copied().collect_tuple().unwrap();

        let mut best = 0;

        while let Some(n) = open.pop_front() {
            if !visited.insert(n.clone()) {
                continue;
            }

            if n.time == 24 {
                // println!("got {:?}", n);
                best = best.max(n.resources[3]);
                continue;
            }

            let mut new = n.clone();
            for j in 0..4 {
                new.resources[j] += new.bots[j];
            }
            new.time += 1;

            let mut idle = true;

            if n.resources[0] >= go && n.resources[2] >= gb {
                let mut new = new.clone();
                new.bots[3] += 1;
                new.resources[0] -= go;
                new.resources[2] -= gb;
                open.push_back(new);
                idle = false;
            } else if n.resources[0] >= bo && n.resources[1] >= bc {
                let mut new = new.clone();
                new.bots[2] += 1;
                new.resources[0] -= bo;
                new.resources[1] -= bc;
                open.push_back(new);
                idle = false;
            } else {

                if n.resources[0] >= oo {
                    let mut new = new.clone();
                    new.bots[0] += 1;
                    new.resources[0] -= oo;
                    open.push_back(new);
                    idle = false;
                }

                if n.resources[0] >= co {
                    let mut new = new.clone();
                    new.bots[1] += 1;
                    new.resources[0] -= co;
                    open.push_back(new);
                    idle = false;
                }

                open.push_back(new.clone());
            }


            // if idle {
            // }
        }

        println!("{} {}", i+1, best);
        part1 += (i+1) * best as usize;
    }

    println!("{} part 1: {}", title, part1);
}

fn run2(title: &str, input: &str) {
    let blueprints: Vec<Vec<u16>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|n| n.parse::<u16>().ok())
                .collect()
        })
        .take(3)
        .collect();

    println!("{:?}", blueprints);

    let mut part2 = 1;

    for (i, bp) in blueprints.iter().enumerate() {
        let mut open = VecDeque::from([
            Node {
                resources: [0, 0, 0, 0],
                bots: [1, 0, 0, 0],
                time: 0,
            }
        ]);

        let mut visited = HashSet::new();

        let (oo, co, bo, bc, go, gb) = bp.iter().copied().collect_tuple().unwrap();

        let mut best = 0;

        while let Some(n) = open.pop_back() {
            if !visited.insert(n.clone()) {
                continue;
            }

            if n.resources[3] > best {
                println!("got {:?}", n);
                best = n.resources[3];
            }

            let end = 32;
            if n.time == end {
                // println!("got {:?}", n);
                // best = best.max(n.resources[3]);
                continue;
            }

            if n.resources[3] + (n.bots[3] + (end - n.time)/2) * (end - n.time) < best {
                continue;
            }

            let mut new = n.clone();
            for j in 0..4 {
                new.resources[j] += new.bots[j];
            }
            new.time += 1;

            let mut idle = true;

            if n.resources[0] >= go && n.resources[2] >= gb {
                let mut new = new.clone();
                new.bots[3] += 1;
                new.resources[0] -= go;
                new.resources[2] -= gb;
                open.push_back(new);
                idle = false;
            }

            if n.resources[0] >= bo && n.resources[1] >= bc {
                let mut new = new.clone();
                new.bots[2] += 1;
                new.resources[0] -= bo;
                new.resources[1] -= bc;
                open.push_back(new);
                idle = false;
            }
            // } else {

                if n.resources[0] >= oo {
                    let mut new = new.clone();
                    new.bots[0] += 1;
                    new.resources[0] -= oo;
                    open.push_back(new);
                    idle = false;
                }

                if n.resources[0] >= co {
                    let mut new = new.clone();
                    new.bots[1] += 1;
                    new.resources[0] -= co;
                    open.push_back(new);
                    idle = false;
                }
            // }

            open.push_back(new.clone());

            // if idle {
            // }
        }

        println!("{} {}", i+1, best);
        part2 *= best as usize;
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("19/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
