// Part 1: 16 mins
// Part 2: 23 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Ban,
    Bor,
    Set,
    Gt,
    Eq,
}
const I: bool = false;
const R: bool = true;

fn consistent(eg: &(Vec<i32>, Vec<i32>, Vec<i32>), op: &(Op, bool, bool)) -> bool {
    let (before, code, after) = eg;

    let mut regs = before.clone();

    let a = if op.1 { before[code[1] as usize] } else { code[1] };
    let b = if op.2 { before[code[2] as usize] } else { code[2] };

    regs[code[3] as usize] = match op.0 {
        Op::Add => a + b,
        Op::Mul => a * b,
        Op::Ban => a & b,
        Op::Bor => a | b,
        Op::Set => a,
        Op::Gt => if a > b { 1 } else { 0 },
        Op::Eq => if a == b { 1 } else { 0 },
    };

    return regs == *after;
}

fn run(title: &str, input: &str) {
    let (egs, prog) = input.split_once("\n\n\n\n").unwrap();
    let egs = egs.split("\n\n").map(|eg| {
        let mut lines = eg.lines();
        let before = lines.next().unwrap()[9..].strip_suffix("]").unwrap().split(", ").map(|n| n.parse::<i32>().unwrap()).collect_vec();
        let code = lines.next().unwrap().split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap()).collect_vec();
        let after = lines.next().unwrap()[9..].strip_suffix("]").unwrap().split(", ").map(|n| n.parse::<i32>().unwrap()).collect_vec();
        (before, code, after)
    }).collect_vec();

    let ops = [
        (Op::Add, R, R),
        (Op::Add, R, I),
        (Op::Mul, R, R),
        (Op::Mul, R, I),
        (Op::Ban, R, R),
        (Op::Ban, R, I),
        (Op::Bor, R, R),
        (Op::Bor, R, I),
        (Op::Set, R, I),
        (Op::Set, I, I),
        (Op::Gt, I, R),
        (Op::Gt, R, I),
        (Op::Gt, R, R),
        (Op::Eq, I, R),
        (Op::Eq, R, I),
        (Op::Eq, R, R),
    ];

    let mut mapping = Vec::new();
    for i in 0..16 {
        mapping.push(HashSet::from(ops.clone()));
    }

    let mut part1 = 0;
    for eg in &egs {
        let c = ops.iter().filter(|op| consistent(eg, op)).count();
        if c >= 3 {
            part1 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    for eg in &egs {
        for op in &ops {
            if !consistent(eg, op) {
                mapping[eg.1[0] as usize].remove(op);
            }
        }
    }
    for k in 0..16 {
        for i in 0..16 {
            if mapping[i].len() == 1 {
                let m = mapping[i].iter().next().unwrap().clone();
                for j in 0..16 {
                    if i != j {
                        mapping[j].remove(&m);
                    }
                }
            }
        }
    }
    println!("{:?}", mapping);

    let mut regs = vec![0, 0, 0, 0];
    for line in prog.lines() {
        let code = line.split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap()).collect_vec();

        let op = mapping[code[0] as usize].iter().next().unwrap();

        let a = if op.1 { regs[code[1] as usize] } else { code[1] };
        let b = if op.2 { regs[code[2] as usize] } else { code[2] };

        regs[code[3] as usize] = match op.0 {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Ban => a & b,
            Op::Bor => a | b,
            Op::Set => a,
            Op::Gt => if a > b { 1 } else { 0 },
            Op::Eq => if a == b { 1 } else { 0 },
        };

    }

    println!("{} part 2: {:?}", title, regs);
}

const INPUT_DEMO: &str = "";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
