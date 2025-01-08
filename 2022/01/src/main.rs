// Part 1: 2 mins
// Part 1+2: 3 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {

    let part1: u32 = input.split("\n\n").map(|elf| {
        elf.lines().map(|s| s.parse::<u32>().unwrap()).sum()
    }).max().unwrap();

    println!("{} part 1: {}", title, part1);

    let mut p: Vec<u32> = input.split("\n\n").map(|elf| {
        elf.lines().map(|s| s.parse::<u32>().unwrap()).sum()
    }).collect_vec();
    p.sort();

    println!("{} part 2: {}", title, p[p.len()-3 ..].iter().sum::<u32>());
}

const INPUT_DEMO: &str = "";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
