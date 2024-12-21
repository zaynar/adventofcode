// Part 1: 2 mins
// Part 1+2: 2 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect_vec();

    for a in &data {
        for b in &data {
            if a + b == 2020 {
                println!("{} part 1: {}", title, a*b);
            }

            for c in &data {
                if a + b + c == 2020 {
                    println!("{} part 2: {}", title, a*b*c);
                }
                }
        }
    }
}

const INPUT_DEMO: &str = "";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
