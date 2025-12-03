// Part 1: 7 mins
// Part 1+2: 21 mins

use std::collections::HashMap;

fn max_jolt(buf: &[u32], n: u32, digs: usize) -> u32 {
    if digs == 0 {
        return n;
    }

    if buf.is_empty() {
        return 0;
    }

    buf.iter()
        .enumerate()
        .map(|(i, c)| max_jolt(&buf[i + 1..], n * 10 + c, digs - 1))
        .max()
        .unwrap()
}

fn max_jolt2(
    cache: &mut HashMap<(usize, usize), Option<u64>>,
    buf: &[u32],
    off: usize,
    digs: usize,
) -> Option<u64> {
    if digs == 0 {
        return Some(0);
    }

    if off == buf.len() {
        return None;
    }

    let key = (off, digs);
    if let Some(r) = cache.get(&key) {
        *r
    } else {
        let r = buf
            .iter()
            .enumerate()
            .skip(off)
            .filter_map(|(i, c)| {
                max_jolt2(cache, buf, i + 1, digs - 1)
                    .map(|n| n + *c as u64 * 10u64.pow((digs - 1) as u32))
            })
            .max();
        cache.insert(key, r);
        r
    }
}

fn run(title: &str, input: &str) {
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for bank in &data {
        let mut cache = HashMap::new();

        // println!(
        //     "{:?} {} {:?}",
        //     bank,
        //     max_jolt(bank, 0, 2),
        //     max_jolt2(&mut cache, bank, 0, 12)
        // );

        part1 += max_jolt(bank, 0, 2);

        part2 += max_jolt2(&mut cache, bank, 0, 12).unwrap();
    }

    println!("{} part 1: {}", title, part1);
    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
