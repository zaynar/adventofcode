// Part 1: 4 mins
// Part 1+2: 10 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (ranges, avail) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let avail: Vec<u64> = avail.lines().map(|n| n.parse().unwrap()).collect();

    let mut part1 = 0;

    for i in &avail {
        if ranges.iter().any(|(a, b)| a <= i && i <= b) {
            part1 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    let edges: Vec<(u64, i32)> = ranges
        .into_iter()
        .flat_map(|(a, b)| [(a, -1), (b, 1)])
        .sorted()
        .collect();

    let mut part2 = 0;
    let mut depth = 0;
    let mut start = 0;

    for (n, dir) in edges {
        // println!("{n} {dir}");
        if depth == 0 {
            assert_eq!(dir, -1);
            start = n;
        }
        depth += dir;
        if depth == 0 {
            part2 += n + 1 - start;
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
