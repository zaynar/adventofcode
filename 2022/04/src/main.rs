// Part 1: 4 mins
// Part 1+2: 5 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            line.split(|c| c == '-' || c == ',').map(|n| n.parse().unwrap()).collect_tuple().unwrap()
        })
        .collect();

    // println!("{:?}", data);

    let mut part1 = 0;
    let mut part2 = 0;
    for (a,b,c,d) in &data {
        if (a >= c && b <= d) || (c >= a && d <= b) {
            part1 += 1;
        }

        if !(b < c || a > d) {
            part2 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
