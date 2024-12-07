use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut part1 = 0;
    for line in input.lines() {
        let mut cmps = line.split("-").collect_vec();
        let last = cmps.pop().unwrap();
        let (id, csum) = last.split_once("[").unwrap();
        let id: u32 = id.parse().unwrap();
        let csum = csum.trim_end_matches("]");

        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in cmps.concat().chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let sorted = counts.iter().sorted_by_key(|(&c, &n)| (-n, c)).collect_vec();
        assert!(sorted.len() >= 5);
        let csum2 = sorted[0..5].iter().map(|(&c, &n)| c).collect::<String>();
        // println!("{:?}", sorted);
        // println!("{} {} {:?}", csum2, csum, cmps);

        if csum2 == csum {
            part1 += id;

            let cmps = cmps.iter().map(|e|
                e.chars().map(|c| ((c as u32 - 'a' as u32 + id) % 26 + 'a' as u32) as u8 as char).collect::<String>()).collect_vec();
            println!("{} {}", cmps.join(" "), id);
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
