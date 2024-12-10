// Part 1: 3 min
// Part 1+2: 4 min

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut data: Vec<u32> = input.trim().chars().map(|n| n.to_digit(10).unwrap()).collect();
    data.push(data[0]);
    let part1 = data.iter().tuple_windows().filter_map(|(a, b)| if a == b { Some(a) } else { None }).sum::<u32>();

    println!("{} part 1: {}", title, part1);

    let mut data: Vec<u32> = input.trim().chars().map(|n| n.to_digit(10).unwrap()).collect();
    let mut part2 = 0;
    for i in 0..data.len() {
        if data[i] == data[(i + data.len() / 2) % data.len()] {
            part2 += data[i];
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "1122";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
