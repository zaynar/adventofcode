// Part 1: 7 mins
// Part 1+2: 61 mins

use rayon::prelude::*;

use std::collections::{HashMap, HashSet};

fn run(title: &str, input: &str) {

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = vec![];
    for line in input.lines() {
        if line.starts_with("mask") {
            mask = line[7..].chars().map(|c| match c {
                'X' => None,
                '0' => Some(0),
                '1' => Some(1),
                _ => panic!()
            }).collect();
        } else {
            let (op, val) = line.split_once(" = ").unwrap();
            let op: u64 = op.strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse().unwrap();
            let val: u64 = val.parse().unwrap();

            let val2 = (0..36).map(|n| mask[35 - n].unwrap_or((val >> n) & 1) << n).sum();
            // println!("{} {} {}", op, val, val2);
            mem.insert(op, val2);
        }
    }

    println!("{} part 1: {}", title, mem.values().sum::<u64>());
}

// Each mask is a box in 36D space
// Partition:
// Start with whole space
// Divide along axis N
// If no write overlaps this space, prune
// Otherwise recurse

fn run3(title: &str, input: &str) {

    let mut writes = vec![];
    // let mut mem: HashMap<(u64, u64), u64> = HashMap::new();

    let mut mask: (u64, u64) = (0, 0);

    for line in input.lines() {
        if line.starts_with("mask") {
            // masks.insert(line[7..].to_owned());

            let m: Vec<u64> = line[7..].chars().map(|c| match c {
                'X' => 0,
                '0' => 1,
                '1' => 1,
                _ => panic!()
            }).collect();
            let v: Vec<u64> = line[7..].chars().map(|c| match c {
                'X' => 0,
                '0' => 0,
                '1' => 1,
                _ => panic!()
            }).collect();
            mask = (
                m.iter().enumerate().map(|(i, n)| (*n as u64) << (35 - i)).sum(),
                v.iter().enumerate().map(|(i, n)| (*n as u64) << (35 - i)).sum(),
            );
            // println!("{:?} {:?} {:?}", m, v, mask);

            // if !masks.contains(&mask) {
            //     masks.push(mask);
            // }

        } else {
            let (op, val) = line.split_once(" = ").unwrap();
            let op: u64 = op.strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse().unwrap();
            let val: u64 = val.parse().unwrap();

            // println!("mask={:x?} op={} {}", mask, op, (op | mask.1) & mask.0);
            writes.push((mask.0, (op | mask.1) & mask.0, val));

            // println!("{} {} {}", op, val, val2);
            // mem.insert(mask, val2);
        }
    }

    writes.reverse();

    let mut open: Vec<(u64, u64)> = vec![(0, (1 << 36) - 1)];

    let mut part2 = 0;
    'OUTER: while let Some((lower, upper)) = open.pop() {

        // println!("{:x} {:x}", lower, upper);

        if lower == upper {
            for (m, v, w) in &writes {
                if (lower & m) == *v {
                    part2 += w;
                    continue 'OUTER;
                }
            }
        }

        let mut ok = false;
        for (m, v, _) in &writes {
            // If mv overlaps *any* value in lower..upper =>
            // For each bit b:
            //   If lower[b] == upper[b]: require m[b]=X OR v[b]=lower[b]
            //
            //    (lower^upper) | !m | !(v ^ lower) = 1...

            // println!("m={:x} v={:x} -- {:x} {:x} {:x} {:x} {:x}", m, v, (lower ^ upper), (!m), !(v ^ lower), (lower ^ upper) | (!m) | !(v ^ lower));

            if ((lower ^ upper) | (!m) | !(v ^ lower)) & ((1 << 36) - 1) == (1 << 36) - 1 {
                ok = true;
                break;
            }
        }

        if !ok { continue; }

        let axis = 1 << (upper ^ lower).ilog2();
        // println!("split {:x}", axis);
        open.push((lower & !axis, upper & !axis));
        open.push((lower | axis, upper | axis));
    }

    println!("part 2: {}", part2);
}

fn run2(title: &str, input: &str) {

    let mut writes = vec![];
    // let mut mem: HashMap<(u64, u64), u64> = HashMap::new();

    let mut mask: (u64, u64) = (0, 0);

    for line in input.lines() {
        if line.starts_with("mask") {
            // masks.insert(line[7..].to_owned());

            let m: Vec<u64> = line[7..].chars().map(|c| match c {
                'X' => 0,
                '0' => 1,
                '1' => 1,
                _ => panic!()
            }).collect();
            let v: Vec<u64> = line[7..].chars().map(|c| match c {
                'X' => 0,
                '0' => 0,
                '1' => 1,
                _ => panic!()
            }).collect();
            mask = (
                m.iter().enumerate().map(|(i, n)| (*n as u64) << (35 - i)).sum(),
                v.iter().enumerate().map(|(i, n)| (*n as u64) << (35 - i)).sum(),
            );
            // println!("{:?} {:?} {:?}", m, v, mask);

            // if !masks.contains(&mask) {
            //     masks.push(mask);
            // }

        } else {
            let (op, val) = line.split_once(" = ").unwrap();
            let op: u64 = op.strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse().unwrap();
            let val: u64 = val.parse().unwrap();

            // println!("mask={:x?} op={} {}", mask, op, (op | mask.1) & mask.0);
            writes.push((mask.0, (op | mask.1) & mask.0, val));

            // println!("{} {} {}", op, val, val2);
            // mem.insert(mask, val2);
        }
    }

    writes.reverse();

    // println!("{} part 1: {}", title, mem.values().sum::<u64>());
    // println!("\n{:#x?}", writes);

    let sum = (0..(1_u64 << 36)).into_par_iter().map(|n| {
        for (m, v, w) in &writes {
            // println!("matched? {} x{:x} x{:x} {}", n, m, v, w);
            if (n & m) == *v {
                // println!("matched {} {:x} {:x} = {}", n, m, v, w);
                return *w;
            }
        }
        // println!("unmatched {:x}", n);
        return 0;
    }).sum::<u64>();

    println!("part 2: {}", sum);
}

const INPUT_DEMO: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

const INPUT_DEMO2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("14/input.txt").unwrap());
    // run2("demo", INPUT_DEMO2);
    // run2("input", &std::fs::read_to_string("14/input.txt").unwrap());
    run3("demo", INPUT_DEMO2);
    run3("input", &std::fs::read_to_string("14/input.txt").unwrap());
}
