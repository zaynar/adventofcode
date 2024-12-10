// Part 1: 1 min
// Part 1+2: 3 min

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| str::parse(n).unwrap())
                .collect()
        })
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for row in data {
        part1 += row.iter().max().unwrap() - row.iter().min().unwrap();
        for v in row.iter().permutations(2) {
            if v[0] % v[1] == 0 {
                part2 += v[0] / v[1];
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "5 1 9 5
7 5 3
2 4 6 8";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
