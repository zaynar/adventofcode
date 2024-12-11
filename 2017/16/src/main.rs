// Part 1: 9 mins
// Part 1+2: 22 mins

use core::str;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn proc(mut progs: Vec<char>, input: &str) -> Vec<char> {
    for cmd in input.trim().split(",") {
        let cmd = cmd.as_bytes();
        if cmd[0] == b's' {
            let n = str::from_utf8(&cmd[1..]).unwrap().parse::<usize>().unwrap();
            progs.rotate_right(n);
        } else if cmd[0] == b'x' {
            let (a, b) = str::from_utf8(&cmd[1..]).unwrap().split_once("/").unwrap();
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            (progs[a], progs[b]) = (progs[b], progs[a]);
        } else if cmd[0] == b'p' {
            progs.iter_mut().for_each(|p| {
                if *p == cmd[1] as char {
                    *p = cmd[3] as char;
                } else if *p == cmd[3] as char {
                    *p = cmd[1] as char;
                }
            });
        } else {
            panic!();
        }
    }

    progs
}

fn run(title: &str, input: &str) {
    let mut progs = (0..16).map(|n| ('a' as u8 + n) as char).collect::<Vec<_>>();

    let progs = proc(progs, input);

    println!("{} part 1: {}", title, progs.iter().collect::<String>());

    let mut seen = HashMap::new();

    let mut progs = (0..16).map(|n| ('a' as u8 + n) as char).collect::<Vec<_>>();
    for i in 0..(1_000_000_000 % 60) {
        if let Some(n) = seen.insert(progs.clone(), i)  {
            println!("loop {}", i-n);
            break;
        }
        if i % 1_000_000 == 0 { println!("{}", i)};
        progs = proc(progs, input);
    }

    println!("{} part 2: {}", title, progs.iter().collect::<String>());
}

// const INPUT_DEMO: &str = "";

fn main() {
    // run("demo", "s1,x3/4,pe/b");
    run("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
