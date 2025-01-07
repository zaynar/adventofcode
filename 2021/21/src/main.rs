// Part 1: 8 mins
// Part 1+2: 28 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run1(title: &str, pos: [u32; 2]) {

    let mut pos = [pos[0] - 1, pos[1] - 1];
    let mut score = [0, 0];

    for t in 0.. {
        let rolls = (t * 6) % 100 + (t * 6 + 1) % 100 + (t * 6 + 2) % 100 + 3;
        // println!("p1 rolls {}", rolls);
        pos[0] += rolls;
        score[0] += pos[0] % 10 + 1;

        if score[0] >= 1000 {
            println!("{} part 1: {}", title, score[1] * (t * 6 + 3));
            break;
        }

        let rolls = (t * 6 + 3) % 100 + (t * 6 + 4) % 100 + (t * 6 + 5) % 100 + 3;
        // println!("p2 rolls {}", rolls);
        pos[1] += rolls;
        score[1] += pos[1] % 10 + 1;

        if score[1] >= 1000 {
            println!("{} part 1: {}", title, score[0] * (t * 6 + 6));
            break;
        }

        // println!("{:?}", score);
    }
}

fn run2(title: &str, pos: [u32; 2]) {

    let pos = [pos[0] - 1, pos[1] - 1];
    let score = [0, 0];

    let mut universes: HashMap<
        (usize, [u32; 2], [u32; 2]), u64
    > = HashMap::new();

    universes.insert((0, pos, score), 1);

    let mut wins = [0, 0];

    while !universes.is_empty() {
        // println!("{:?}", universes.len());
        let mut nu = HashMap::new();

        for ((player, pos, score), count) in universes {
            for ds in (0..3).map(|_| 1..=3).multi_cartesian_product() {
                let mut pos = pos;
                let mut score = score;
                pos[player] += ds[0] + ds[1] + ds[2];
                score[player] += pos[player] % 10 + 1;

                if score[player] >= 21 {
                    wins[player] += count;
                    continue;
                }

                *nu.entry(((player + 1) % 2, pos, score)).or_insert(0) += count;
            }
        }

        universes = nu;
    }

    println!("{} part 2: {:?} {}", title, wins, wins.iter().max().unwrap());
}

fn main() {
    // run1("demo", [4, 8]);
    // run1("input", [6, 8]);
    run2("demo", [4, 8]);
    run2("input", [6, 8]);
}
