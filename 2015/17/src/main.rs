use itertools::Itertools;

fn run(title: &str, input: &str, target: u32) {
    let data: Vec<u32> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut part1 = 0;
    let mut min = usize::MAX;
    for p in data.iter().powerset() {
        if p.iter().copied().sum::<u32>() == target {
            // println!("{:?}", p);
            part1 += 1;
            min = min.min(p.len());
        }
    }

    let mut part2 = 0;
    for p in data.iter().powerset() {
        if p.len() == min && p.iter().copied().sum::<u32>() == target {
            // println!("{:?}", p);
            part2 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "20
15
10
5
5
";

fn main() {
    run("demo", INPUT_DEMO, 25);
    run("input", &std::fs::read_to_string("17/input.txt").unwrap(), 150);
}
