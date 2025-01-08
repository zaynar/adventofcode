// Part 1: 4 mins
// Part 1+2: 8 mins

use std::collections::HashSet;

use itertools::Itertools;

fn run(title: &str, input: &str) {

    let mut part1 = 0;

    for line in input.lines() {
        let a: HashSet<char> = HashSet::from_iter(line[0..line.len()/2].chars());
        let b: HashSet<char> = HashSet::from_iter(line[line.len()/2..].chars());
        for &i in a.intersection(&b) {
            part1 += match i {
                'a'..='z' => 1 + (i as u8 - 'a' as u8) as u32,
                'A'..='Z' => 27 + (i as u8 - 'A' as u8) as u32,
                _ => panic!(),
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;

    for mut line in &input.lines().chunks(3) {
        let a: HashSet<char> = HashSet::from_iter(line.next().unwrap().chars());
        let b: HashSet<char> = HashSet::from_iter(line.next().unwrap().chars());
        let c: HashSet<char> = HashSet::from_iter(line.next().unwrap().chars());
        for &i in HashSet::from_iter(a.intersection(&b).copied()).intersection(&c) {
            part2 += match i {
                'a'..='z' => 1 + (i as u8 - 'a' as u8) as u32,
                'A'..='Z' => 27 + (i as u8 - 'A' as u8) as u32,
                _ => panic!(),
            }
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
