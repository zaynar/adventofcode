// Part 1: 3 mins
// Part 1+2: 26 mins

use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let lines = input.lines().collect_vec();
    let mut lines = lines.iter().map(|line| line.split_once(" ").unwrap()).collect_vec();

    let mut pc: i64 = 0;
    let mut regs: HashMap<String, i64> = HashMap::new();
    for c in 'a'..='h' {
        regs.insert(c.to_string(), 0);
    }
    regs.insert("a".to_string(), 1);

    let mut part1 = 0;

    while pc < lines.len() as i64 {
        let (op, mut r) = lines[pc as usize];

        if let Some(n) = r.find("#") {
            r = &r[(n+1)..];
        }

        match op {
            "set" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(x.to_owned(), if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                pc += 1;
            }
            "sub" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(x.to_owned(), regs[x] - if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                pc += 1;
            }
            "mul" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(x.to_owned(), regs[x] * if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                pc += 1;
                part1 += 1;
            }
            "jnz" => {
                let (x, y) = r.split_once(" ").unwrap();
                let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };
                let y = if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() };
                if x != 0 {
                    pc += y;
                } else {
                    pc += 1;
                }
            }
            _ => panic!()
        }
    }

    println!("{} part 1: {}", title, part1);
    println!("{} part 2: {}", title, regs["h"]);
}

fn run2() {
    let mut b = 79;
    let mut c = 79;

    b = b*100 + 100000;
    c = b + 17000;

    let mut h = 0;

    loop {
        println!("{} {}", b, h);
        let mut f = 1;

        for d in 2..b {
            // for e in 2..b {
            //     if d*e == b {
            //         f = 0;
            //         break;
            //     }
            // }
            if b % d == 0 {
                f = 0;
                break;
            }
        }

        if f == 0 { h += 1; }

        if b == c { break; }
        b += 17;
    }

    println!("part 2: {}", h);
}

fn main() {
    // run("input", &std::fs::read_to_string("23/input.txt").unwrap());
    // run("input", &std::fs::read_to_string("23/input-mod.txt").unwrap());

    run2();
}
