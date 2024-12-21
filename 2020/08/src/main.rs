// Part 1: 5 mins
// Part 1+2: 7 mins

use std::collections::HashSet;

fn run(title: &str, input: &str) {
    let data: Vec<(String, i64)> = input
        .lines()
        .map(|line| {
            let (op, x) = line.split_once(" ").unwrap();
            let x = x.parse().unwrap();
            (op.to_owned(), x)
        })
        .collect();

    // println!("{:?}", data);

    {
        let mut seen = HashSet::new();

        let mut acc = 0;
        let mut ip = 0_i64;
        while (ip as usize) < data.len() {
            let (op, x) = &data[ip as usize];
            // println!("{} {} {}", ip, op, x);

            if !seen.insert(ip) {
                println!("{} part 1: {}", title, acc);
                break;
            }

            match op.as_str() {
                "acc" => {
                    acc += x;
                    ip += 1;
                }
                "jmp" => {
                    ip += x;
                }
                "nop" => {
                    ip += 1;
                }
                _ => panic!()
            }
        }
    }

    for fix in 0..data.len() {
        let mut seen = HashSet::new();

        let mut acc = 0;
        let mut ip = 0_i64;
        while (ip as usize) < data.len() {
            let (mut op, x) = data[ip as usize].clone();
            // println!("{} {} {}", ip, op, x);

            if ip == fix as i64 {
                if op == "nop" { op = "jmp".to_owned(); }
                else if op == "jmp" { op = "nop".to_owned(); }
            }

            if !seen.insert(ip) {
                break;
            }

            match op.as_str() {
                "acc" => {
                    acc += x;
                    ip += 1;
                }
                "jmp" => {
                    ip += x;
                }
                "nop" => {
                    ip += 1;
                }
                _ => panic!()
            }

            if ip == data.len() as i64 {
                println!("{} part 2: {}", title, acc);
                return;
            }
        }
    }
}

const INPUT_DEMO: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
