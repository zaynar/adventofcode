use itertools::Itertools;

fn run(title: &str, input: &str) {
    let area: u32 = input
        .lines()
        .map(|line| {
            let mut d = line
                .split("x")
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec();
            d.sort();
            (d[0] * d[1] + d[1] * d[2] + d[2] * d[0]) * 2 + d[0] * d[1]
        })
        .sum();

    println!("{} part 1: {}", title, area);

    let area: u32 = input
        .lines()
        .map(|line| {
            let mut d = line
                .split("x")
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec();
            d.sort();
            2 * (d[0] + d[1]) + d[0] * d[1] * d[2]
        })
        .sum();

    println!("{} part 2: {}", title, area);
}

fn main() {
    run("demo 1", "2x3x4");
    run("demo 2", "1x1x10");
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
