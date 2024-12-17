// Part 1: 15 mins
// Part 2: 67 mins

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn run(title: &str, mut a: i64, mut b: i64, mut c: i64, opcodes: &[i64]) {

    let mut pc: i64 = 0;

    let mut outputs = Vec::new();

    while pc >= 0 && pc < opcodes.len() as i64 {
        let op = opcodes[pc as usize];
        let operand = opcodes[(pc+1) as usize];
        let r = if [0, 2, 5, 6, 7].contains(&op) {
            match operand {
                0|1|2|3 => operand,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!()
            }
        } else {
            operand
        };
        // println!("op={} {} r={} pc={} a={} b={} c={}", op, operand, r, pc, a, b, c);
        match op {
            0 => { // adv
                a = a / 2_i64.pow(r as u32);
                pc += 2;
            }
            1 => { // bxl
                b = b ^ r;
                pc += 2;
            }
            2 => { // bst
                b = r % 8;
                pc += 2;
            }
            3 => { // jnz
                if a == 0 {
                    pc += 2;
                } else {
                    pc = r;
                }
            }
            4 => { // bxc
                b = b ^ c;
                pc += 2;
            }
            5 => { // out
                outputs.push(r % 8);
                pc += 2;
            }
            6 => { // bdv
                b = a / (1 << r);
                pc += 2;
            }
            7 => { // cdv
                c = a / (1 << r);
                pc += 2;
            }
            _ => panic!()
        }
    }

    println!("{} part 1: {}", title, outputs.iter().map(|n| n.to_string()).join(","));
}

fn run2(title: &str, mut a: i64, mut b: i64, mut c: i64, opcodes: &[i64]) -> bool {

    let mut pc: i64 = 0;

    let mut output_idx = 0;

    let mut cycles = 0;

    while pc >= 0 && pc < opcodes.len() as i64 {
        let op = opcodes[pc as usize];
        let operand = opcodes[(pc+1) as usize];
        let r = if [0, 2, 5, 6, 7].contains(&op) {
            match operand {
                0|1|2|3 => operand,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!()
            }
        } else {
            operand
        };
        // println!("op={} {} r={} pc={} a={} b={} c={}", op, operand, r, pc, a, b, c);
        match op {
            0 => { // adv
                a = a / 2_i64.pow(r as u32);
                pc += 2;
            }
            1 => { // bxl
                b = b ^ r;
                pc += 2;
            }
            2 => { // bst
                b = r % 8;
                pc += 2;
            }
            3 => { // jnz
                if a == 0 {
                    pc += 2;
                } else {
                    pc = r;
                }
            }
            4 => { // bxc
                b = b ^ c;
                pc += 2;
            }
            5 => { // out
                println!("OUT2 {} r={} a={} b={} c={}", r % 8, r, a, b, c);
                if output_idx >= opcodes.len() {
                    return false;
                }
                if opcodes[output_idx] != r % 8 {
                    // return false;
                }
                output_idx += 1;
                pc += 2;
            }
            6 => { // bdv
                b = a / (1 << r);
                pc += 2;
            }
            7 => { // cdv
                c = a / (1 << r);
                pc += 2;
            }
            _ => panic!()
        }

        cycles += 1;
        if cycles > 1_000 {
            break;
        }
    }

    return output_idx == opcodes.len();
}



/*
2,4, bst a
1,2, bxl 2
7,5, cdv b
4,5, bxc
0,3, adv 3
1,7, bxl 7
5,5, out b
3,0  jnz 0

loop {
    b = a % 8
    b ^= 2
    c = a >> b
    b ^= c
    a = a >> 3
    b ^= 7
    out b
}
*/

fn run3(title: &str, mut a: i64,  opcodes: &[i64]) -> Option<usize> {
    let mut i = a;
    let mut b: i64 = 0;
    let mut c: i64 = 0;
    let mut output_idx = 0;

    loop {
        // println!("a={} b={} c={}", a, b, c);
        b = (a % 8) ^ 2;
        c = a >> b;
        // println!("shr {}", b);
        b ^= c;
        a >>= 3;
        b ^= 7;

        // println!("OUT3 r={} a={} b={} c={}", b, a, b, c);
        // outputs.push(b % 8);
        if opcodes[output_idx] != b % 8 {
            return Some(output_idx);
        }

        output_idx += 1;

        if a == 0 {
            if output_idx == opcodes.len() {
                return Some(output_idx);
            } else {
                return None;
            }
        } else {
            if output_idx == opcodes.len() {
                return None;
            }
        }
    }
}

fn expand(opcodes: &[i64]) {
    let mut open = VecDeque::new();
    open.push_back((0, 0));

    let mut best = i64::MAX;

    let mut i = 0;

    let mut seen = HashSet::new();

    while let Some((digit, input)) = open.pop_front() {

        if i % 100 == 0 {
            // println!("{} {} {}", digit, input, open.len());
        }
        i += 1;

        for x in 0..1024 {
            let i = input ^ (x << (digit * 3));
            if let Some(got) = run3("demo", i, opcodes) {
                if got == opcodes.len() {
                    if i < best {
                        println!("part 2: {} d={}", i, digit);
                        println!("### {}", run2("", i, 0, 0, opcodes));
                        best = i;
                    }
                }
                if got as i64 > digit {
                    if seen.insert((digit + 1, i)) {
                        open.push_back((digit + 1, i));
                    }
                }
            }
        }
    }
}
fn main() {
    run("demo", 729, 0, 0, &[0,1,5,4,3,0]);
    run("input", 22817223, 0, 0, &[2,4,1,2,7,5,4,5,0,3,1,7,5,5,3,0]);

    expand(&[2,4,1,2,7,5,4,5,0,3,1,7,5,5,3,0]);
}
