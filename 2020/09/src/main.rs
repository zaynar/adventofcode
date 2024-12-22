// Part 1: 4 mins
// Part 1+2: 6 mins

use itertools::Itertools;

fn run(title: &str, input: &str, preamble: usize) {
    let data: Vec<i64> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut found = 0;
    for i in preamble..data.len() {
        let ok = data[(i-preamble)..i].iter().tuple_combinations().any(|(a, b)|
            a + b == data[i]
        );
        if !ok {
            found = data[i];
            println!("{} part 1: {}", title, data[i]);
            break;
        }
    }

    for a in 0..data.len() {
        for b in (a+1)..data.len() {
            if data[a..b].iter().sum::<i64>() == found {
                println!("{} part 2: {}", title, data[a..b].iter().min().unwrap() + data[a..b].iter().max().unwrap());
            }
        }
    }
}

const INPUT_DEMO: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

fn main() {
    run("demo", INPUT_DEMO, 5);
    run("input", &std::fs::read_to_string("09/input.txt").unwrap(), 25);
}
