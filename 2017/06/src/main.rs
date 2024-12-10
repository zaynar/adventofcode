// Part 1: 7 mins
// Part 1+2: 8 mins

use std::collections::HashMap;

fn run(title: &str, input: &str) {
    let mut data: Vec<i32> = input.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

    let mut seen = HashMap::new();

    for k in 0.. {
        println!("{:?}", data);
        if let Some(k2) = seen.insert(data.clone(), k) {
            println!("{} part 1: {}", title, k);
            println!("{} part 2: {}", title, k - k2);
            break;
        }
        let min = data.iter().max().unwrap();
        let (i, &n) = data.iter().enumerate().find(|(i, n)| *n == min).unwrap();
        data[i] = 0;
        for j in 0..(n as usize) {
            let len = data.len();
            data[(i + j + 1) % len] += 1;
        }
    }
}

const INPUT_DEMO: &str = "0 2 7 0";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
