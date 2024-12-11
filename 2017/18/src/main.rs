// Part 1: 8 mins
// Part 1+2: 41 mins

use std::{collections::HashMap, thread};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let lines = input.lines().collect_vec();
    let mut lines = lines.iter().map(|line| line.split_once(" ").unwrap()).collect_vec();

    let mut pc: i64 = 0;
    let mut regs: HashMap<String, i64> = HashMap::new();
    for c in 'a'..='z' {
        regs.insert(c.to_string(), 0);
    }

    let mut snd = 0;

    while pc < lines.len() as i64 {
        let (op, r) = lines[pc as usize];

        match op {
            "snd" => {
                let x = r;
                let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };
                snd = x;
                pc += 1;
            }
            "set" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(x.to_owned(), if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                pc += 1;
            }
            "add" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(x.to_owned(), regs[x] + if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                pc += 1;
            }
            "mul" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(x.to_owned(), regs[x] * if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                pc += 1;
            }
            "mod" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(x.to_owned(), regs[x] % if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                pc += 1;
            }
            "rcv" => {
                let x = r;
                let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };
                if x != 0 {
                    println!("{} part 1: {}", title, snd);
                    return;
                }
                pc += 1;
            }
            "jgz" => {
                let (x, y) = r.split_once(" ").unwrap();
                let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };
                let y = if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() };
                if x > 0 {
                    pc += y;
                } else {
                    pc += 1;
                }
            }
            _ => panic!()
        }
    }

    println!("{} part 2: {}", title, "TODO");
}

fn run2(title: &str, input: &str) {
    let lines = input.lines().collect_vec();
    let lines = lines.iter().map(|line| { let (op, r) = line.split_once(" ").unwrap(); (op.to_owned(), r.to_owned()) }).collect_vec();

    let (tx0, rx0) = crossbeam_channel::unbounded();
    let (tx1, rx1) = crossbeam_channel::unbounded();
    let mut txs = [tx0, tx1];
    let mut rxs = [rx0, rx1];

    let mut hs = (0..2).map(|k| thread::spawn({
        let title = title.to_string();
        let lines = lines.clone();
        let tx = txs[k ^ 1].clone();
        let rx = rxs[k].clone();

        move || {
            let mut sends = 0;

            let mut pc: i64 = 0;
            let mut regs: HashMap<String, i64> = HashMap::new();
            for c in 'a'..='z' {
                regs.insert(c.to_string(), 0);
            }

            regs.insert("p".to_owned(), k as i64);

            println!("{} start", k);

            while pc < lines.len() as i64 {
                let (op, r) = &lines[pc as usize];

                match op.as_str() {
                    "snd" => {
                        let x = r;
                        let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };
                        // println!("{} send {}", k, x);
                        tx.send(x).unwrap();
                        sends += 1;
                        pc += 1;
                    }
                    "set" => {
                        let (x, y) = r.split_once(" ").unwrap();
                        regs.insert(x.to_owned(), if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                        pc += 1;
                    }
                    "add" => {
                        let (x, y) = r.split_once(" ").unwrap();
                        regs.insert(x.to_owned(), regs[x] + if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                        pc += 1;
                    }
                    "mul" => {
                        let (x, y) = r.split_once(" ").unwrap();
                        regs.insert(x.to_owned(), regs[x] * if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                        pc += 1;
                    }
                    "mod" => {
                        let (x, y) = r.split_once(" ").unwrap();
                        regs.insert(x.to_owned(), regs[x] % if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() });
                        pc += 1;
                    }
                    "rcv" => {
                        // println!("{} rcv", k);
                        println!("{} part 2: {} {:?} {:?}", title, k, sends, regs);

                        let n = rx.recv().unwrap();
                        regs.insert(r.to_owned(), n);
                        // println!("{} rcvd {}", k, n);
                        pc += 1;
                    }
                    "jgz" => {
                        let (x, y) = r.split_once(" ").unwrap();
                        let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };
                        let y = if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() };
                        if x > 0 {
                            pc += y;
                        } else {
                            pc += 1;
                        }
                    }
                    _ => panic!()
                }
            }

            println!("end {} part 2: {} {:?}", title, k, sends);
    }})).collect_vec();
    hs.into_iter().for_each(|t| t.join().unwrap());
}

const INPUT_DEMO: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("18/input.txt").unwrap());
//     run2("demo", "snd 1
// snd 2
// snd p
// rcv a
// rcv b
// rcv c
// rcv d
// ");
    run2("input", &std::fs::read_to_string("18/input.txt").unwrap());
}
