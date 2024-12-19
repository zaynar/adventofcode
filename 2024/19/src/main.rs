// Part 1: 10 mins
// Part 1+2: 13 mins

use std::collections::HashMap;

use itertools::Itertools;

fn make<'a>(pattern: &'a str, towels: &Vec<&str>, cache: &'a mut HashMap<String, bool>) -> bool {
    if pattern.is_empty() {
        return true;
    }

    if let Some(r) = cache.get(pattern) {
        return *r;
    }

    let r = towels.iter().any(|t| pattern.starts_with(t) && make(&pattern[t.len()..], towels, cache));
    // println!(" make {} = {}", pattern, r);
    cache.insert(pattern.to_owned(), r);
    r
}

fn make2<'a>(pattern: &'a str, towels: &Vec<&str>, cache: &'a mut HashMap<String, u64>) -> u64 {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(r) = cache.get(pattern) {
        return *r;
    }

    let r = towels.iter().map(|t| if pattern.starts_with(t) { make2(&pattern[t.len()..], towels, cache) } else { 0 }).sum::<u64>();
    // println!(" make {} = {}", pattern, r);
    cache.insert(pattern.to_owned(), r);
    r
}

fn run(title: &str, input: &str) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect_vec();
    let patterns = patterns.trim().split("\n").collect_vec();

    // println!("{:?}", towels);
    // println!("{:?}", patterns);

    let mut part1 = 0;
    for p in &patterns {
        let mut cache = HashMap::new();
        // println!("{} {}", p, make(p, &towels, &mut cache));
        if make(p, &towels, &mut cache) {
            part1 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;
    for p in &patterns {
        let mut cache = HashMap::new();
        // println!("{} {}", p, make(p, &towels, &mut cache));
        part2 += make2(p, &towels, &mut cache);
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
