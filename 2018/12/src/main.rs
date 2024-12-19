// Part 1: 10 mins
// Part 1+2: 19 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (initial, rules) = input.split_once("\n\n").unwrap();
    let initial = initial.split_once(": ").unwrap().1.chars().collect_vec();

    let mut ok = HashSet::new();
    for line in rules.lines() {
        if line.ends_with("#") {
            ok.insert(line[0..5].chars().collect_vec());
        }
    }

    // println!("{:?}", initial);
    // println!("{:?}", ok);

    let gens = 100;
    let offset = gens * 2;

    let mut state = vec!['.'; offset];
    state.append(&mut initial.clone());
    state.append(&mut vec!['.'; offset]);

    let mut seen = HashMap::new();

    for i in 0..gens {
        let trim = state.iter().rev().skip_while(|c| **c == '.').collect_vec().iter().rev().skip_while(|c| ***c == '.').copied().copied().collect_vec();
        if let Some(n) = seen.get(&trim) {
            println!("-- loop {} {}", n, i);
        } else {
            seen.insert(trim.clone(), i);
        }
        let mut new = state.windows(5).map(|w| if ok.contains(w) { '#' } else { '.' }).collect_vec();
        new.insert(0, '.');
        new.insert(0, '.');
        new.push('.');
        new.push('.');
        state = new;

        // println!("{}", state.iter().collect::<String>());

        if i == 19 {
            let part1 = state.iter().enumerate().map(|(i, c)| if *c == '#' { i as i32 - offset as i32 } else { 0 }).sum::<i32>();
            println!("{} part 1: {}", title, part1);

        }
    }

    let mut part2 = state.iter().enumerate().map(|(i, c)| if *c == '#' { i as i32 - offset as i32 } else { 0 }).sum::<i32>() as i64;
    part2 += (50_000_000_000 - gens as i64) * state.iter().filter(|c| **c == '#').count() as i64;

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
