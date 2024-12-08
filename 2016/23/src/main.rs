use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut pc: i32 = 0;
    let lines = input.lines().collect_vec();
    let mut regs: HashMap<&str, i32> = HashMap::new();
    regs.insert("a", 12);
    regs.insert("b", 0);
    regs.insert("c", 0);
    regs.insert("d", 0);

    let mut lines = lines.iter().map(|line| line.split_once(" ").unwrap()).collect_vec();

    let mut toggled = HashSet::new();

    let mut i = 0;

    while pc < lines.len() as i32 {
        let (op, r) = lines[pc as usize];

        i += 1;
        if i % 10_000_000 == 0 {
            println!("{}: {} {:?} {:?} --- {:?} --- {:?}", i, pc, regs, lines[pc as usize], lines, toggled);
        }

        match op {
            /*
            Replace input.txt line 5-11 with

            mad b d a
            cpy 0 c
            cpy 0 d
            nop _
            nop _
            nop _
             */
            "add" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(y, regs[y] + if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() });
                pc += 1;
            }
            "mad" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert("a", regs["b"] * regs["d"]);
                pc += 1;
            }
            "nop" => {
                pc += 1;
            }

            "cpy" => {
                let (x, y) = r.split_once(" ").unwrap();
                if regs.contains_key(y) {
                    regs.insert(y, if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() });
                }
                pc += 1;
            }
            "inc" => {
                if regs.contains_key(r) {
                    regs.insert(r, regs[r] + 1);
                }
                pc += 1;
            }
            "dec" => {
                if regs.contains_key(r) {
                    regs.insert(r, regs[r] - 1);
                }
                pc += 1;
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
            "tgl" => {
                let x = r;
                let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };

                if pc + x >= 0 && pc + x < lines.len() as i32 {
                    if toggled.insert(pc + x) {
                        println!("toggle {:?}", toggled);
                    }

                    lines[(pc + x) as usize].0 = match lines[(pc + x) as usize].0 {
                        "cpy" => "jnz",
                        "inc" => "dec",
                        "dec" => "inc",
                        "jnz" => "cpy",
                        "tgl" => "inc",
                        _ => panic!(),
                    };
                }

                pc += 1;
            }
            _ => panic!()
        }
    }

    println!("{:?}", regs);

    println!("{} part N: {}", title, regs["a"]);
}

const INPUT_DEMO: &str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a
";

fn main() {
    run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("23/input.txt").unwrap());
    run("input", &std::fs::read_to_string("23/input-mod.txt").unwrap());
}
