// Part 1: 6 mins
// Part 1+2: 14 mins

use std::collections::HashMap;

use itertools::Itertools;

fn count(target: i64, cur: i64, remaining: Vec<i64>, cache: &mut HashMap<(i64, Vec<i64>), usize>) -> usize {
    if cur >= target - 3 {
        return 1;
    }

    if let Some(c) = cache.get(&(cur, remaining.clone())) {
        return *c;
    }

    let mut c = 0;
    for i in 0..remaining.len() {
        let d = remaining[i] - cur;
        if d >= 1 && d <= 3 {
            c += count(target, remaining[i], remaining[(i+1)..].to_vec(), cache);
        }
    }

    cache.insert((cur, remaining), c);
    c
}

fn run(title: &str, input: &str) {
    let data: Vec<i64> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect();

    let builtin = data.iter().max().unwrap() + 3;

    {
        let mut part1_1 = 0;
        let mut part1_3 = 0;

        let mut data = data.clone();
        data.sort();
        data.push(builtin);
        let mut prev = 0;
        for d in &data {
            // println!("# {} {}", prev, d);
            let diff = d - prev;
            assert!(diff >= 1 && diff <= 3);
            if diff == 1 {
                part1_1 += 1;
            }
            if diff == 3 {
                part1_3 += 1;
            }
            prev = *d;
        }

        println!("{} part 1: {}", title, part1_1 * part1_3);
    }

    let mut cache = HashMap::new();
    println!("{} part 2: {}", title, count(builtin, 0, data.clone(), &mut cache));
}

const INPUT_DEMO: &str = "16
10
15
5
1
11
7
19
6
12
4
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
