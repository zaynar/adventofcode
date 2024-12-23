// Part 1: 17 mins
// Part 1+2: 21 mins

use itertools::Itertools;

fn bits(packet: &[u32]) -> usize {
    let mut n = 0;
    for b in packet {
        n = (n << 1) | b;
    }
    n as usize
}

fn parse(packet: &[u32], part1: &mut usize) -> (usize, usize) {
    let v = bits(&packet[0..3]);
    let t = bits(&packet[3..6]);
    let mut i = 6;

    *part1 += v;

    // assert!(*part1 < 100);

    if t == 4 {
        let mut n = 0;
        loop {
            let last = packet[i];
            n = (n << 4) | bits(&packet[(i+1)..(i+5)]);
            i += 5;

            if last == 0 {
                break;
            }
        }
        // println!("literal {}", n);

        return (i, n);
    } else {
        let mut vals = vec![];
        let typeid = packet[i];
        if typeid == 0 {
            let len = bits(&packet[(i+1)..(i+16)]);
            i += 16;
            // println!("t0 {}", len);

            let mut subl = 0;
            while subl < len {
                let (p, n) = parse(&packet[(i+subl)..], part1);
                vals.push(n);
                subl += p;
            }
            assert_eq!(subl, len);
            i += subl;

        } else {
            let count = bits(&packet[(i+1)..(i+12)]);
            i += 12;
            // println!("t1 {}", count);

            let mut subl = 0;
            for j in 0..count {
                let (p, n) = parse(&packet[(i+subl)..], part1);
                vals.push(n);
                subl += p;
            }
            i += subl;
        }

        let r = match t {
            0 => vals.iter().sum::<usize>(),
            1 => vals.iter().product(),
            2 => *vals.iter().min().unwrap(),
            3 => *vals.iter().max().unwrap(),
            5 => if vals[0] > vals[1] { 1 } else { 0 },
            6 => if vals[0] < vals[1] { 1 } else { 0 },
            7 => if vals[0] == vals[1] { 1 } else { 0 },
            _ => panic!(),
        };

        return (i, r);
    }
}

fn run(title: &str, input: &str) {
    let bits = input.trim().chars().map(|c| {
        let n = c.to_digit(16).unwrap();
        vec![(n >> 3) & 1, (n >> 2) & 1, (n >> 1) & 1, n & 1]
    }).concat();

    // println!("{:?}", bits);
    let mut part1 = 0;
    let (_, result) = parse(&bits, &mut part1);

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, result);
}

fn main() {
    // run("demo0", "D2FE28");
    // run("demo1", "38006F45291200");
    // run("demo2", "A0016C880162017C3686B18A3D4780");
    run("demo3", "9C0141080250320F1802104A08");
    run("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
