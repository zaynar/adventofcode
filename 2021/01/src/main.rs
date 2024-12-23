// Part 1: 3 mins
// Part 1+2: 4 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut part1 = 0;
    for (a, b) in input.lines().tuple_windows() {
        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();
        if a < b {
            part1 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;
    for (a, b) in input.lines().map(|n| n.parse::<u32>().unwrap()).tuple_windows().map(|(a, b, c)| a+b+c).tuple_windows() {
        if a < b {
            part2 += 1;
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "199
200
208
210
200
207
240
269
260
263
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
