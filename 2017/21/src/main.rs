// Part 1: 70 mins
// Part 1+2: 71 mins

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct Rule {
    from: Vec<bool>,
    to: Vec<bool>,
}

fn hflip(m: &Vec<bool>) -> Vec<bool> {
    let (w, h) = if m.len() == 4 { (2, 2) } else { assert_eq!(m.len(), 9); (3, 3) };
    let mut ret = Vec::new();
    for y in 0..h {
        for x in 0..w {
            ret.push(m[w-1-x + y*w]);
        }
    }
    ret
}

fn vflip(m: &Vec<bool>) -> Vec<bool> {
    let (w, h) = if m.len() == 4 { (2, 2) } else { assert_eq!(m.len(), 9); (3, 3) };
    let mut ret = Vec::new();
    for y in 0..h {
        for x in 0..w {
            ret.push(m[x + (h-1-y)*w]);
        }
    }
    ret
}

fn rot90(m: &Vec<bool>) -> Vec<bool> {
    let (w, h) = if m.len() == 4 { (2, 2) } else { assert_eq!(m.len(), 9); (3, 3) };
    let mut ret = Vec::new();
    for y in 0..h {
        for x in 0..w {
            ret.push(m[y + (h-1-x)*w]);
        }
    }
    ret
}

// 3x3 e-> 4x4
// 4x4 s-> 4x(2x2) e-> 4x(3x3) = 6x6 - need to merge
// 6x6 s-> 9x(2x2) e-> 9x(3x3) = 9x9 - independent chunks
// 9x9 s-> 9x(3x3) e-> 9x(4x4) = 12x12
// 12x12 s-> 36x(2x2) e-> 36x(3x3) = 18x18
// 18x18 -> 27x27 -> 36x36 -> 54x54 ->
// even, even, odd

fn count(rules: &HashMap<Vec<bool>, Vec<bool>>, grid: &Vec<bool>, reps: usize) -> usize {
    let s = (grid.len() as f64).sqrt() as usize;

    // println!("r={} {}^2 {}", reps, s, fmtgrid(&grid));

    if reps == 0 {
        return grid.iter().filter(|c| **c).count();
    }

    // 9x9 -> 3x3
    if s == 9 {
        let mut sum = 0;
        for y in 0..s/3 {
            for x in 0..s/3 {
                let sub = (0..3).map(|dy| (0..3).map(|dx|
                    grid[x*3 + dx + (y*3 + dy)*s]
                ).collect_vec()).concat();
                sum += count(rules, &sub, reps);
            }
        }
        return sum;
    }

    // 6x6 -> 9x9
    // 4x4 -> 6x6
    if s % 2 == 0 {
        let mut new = Vec::new();
        let ns = (s/2) * 3;
        new.resize(ns*ns, false);
        for y in 0..s/2 {
            for x in 0..s/2 {
                let sub = (0..2).map(|dy| (0..2).map(|dx|
                    grid[x*2 + dx + (y*2 + dy)*s]
                ).collect_vec()).concat();
                let sub = rules.get(&sub).unwrap();
                (0..3).for_each(|dy| (0..3).for_each(|dx| {
                    new[x*3 + dx + (y*3 + dy)*ns] = sub[dx + dy*3];
                }));
            }
        }
        return count(rules, &new, reps - 1);
    }

    // 3x3 -> 4x4
    assert!(s == 3);
    // println!("  --> {}", fmtgrid(rules.get(grid).unwrap()));
    return count(rules, rules.get(grid).unwrap(), reps - 1);

}

fn fmtgrid(m: &Vec<bool>) -> String {
    let s = (m.len() as f64).sqrt() as usize;
    let (w, h) = (s, s);
    (0..h).map(|y| (0..w).map(|x| if m[x + y*w] { "#" } else {"."}).join("")).join("/")
}

fn fmtgridn(m: &Vec<bool>) -> String {
    let s = (m.len() as f64).sqrt() as usize;
    let (w, h) = (s, s);
    (0..h).map(|y| (0..w).map(|x| if m[x + y*w] { "#" } else {"."}).join("")).join("\n")

}

fn run(title: &str, input: &str, reps: usize) {
    let data: Vec<Rule> = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            let from = from.chars().filter_map(|c| match c { '.' => Some(false), '#' =>Some(true), _ => None}).collect_vec();
            let to = to.chars().filter_map(|c| match c { '.' => Some(false), '#' =>Some(true), _ => None}).collect_vec();
            Rule { from, to }
        })
        .collect();

    let mut rules = HashMap::new();
    for Rule { from, to } in &data {
        let trans = hflip(&vflip(from));
        for f in [from.clone(), hflip(from), vflip(from), trans.clone(), rot90(from), rot90(&trans),
        hflip(&rot90(&trans)),
        // vflip(&rot90(&trans)),
        ] {
            // println!("{}\n", fmtgridn(&f));
            if let Some(prev) = rules.insert(f, to.clone()) {
                assert_eq!(prev, *to);
            }
        }
    }

    let grid = vec![false, true, false, false, false, true, true, true, true];

    println!("{} part N: {}", title, count(&rules, &grid, reps));
}

const INPUT_DEMO: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
";

fn main() {
    run("demo", INPUT_DEMO, 2);
    run("input", &std::fs::read_to_string("21/input.txt").unwrap(), 5);
    run("input", &std::fs::read_to_string("21/input.txt").unwrap(), 18);
}
