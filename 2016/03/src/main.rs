use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| str::parse(n).unwrap())
                .collect()
        })
        .collect();

    let part1 = data.iter().filter(|t| {
        t[0] + t[1] > t[2] &&
        t[1] + t[2] > t[0] &&
        t[2] + t[0] > t[1]
    }).count();

    println!("{} part 1: {}", title, part1);

    let data = data.chunks_exact(3).map(|c| {
        [
            [ c[0][0], c[1][0], c[2][0] ],
            [ c[0][1], c[1][1], c[2][1] ],
            [ c[0][2], c[1][2], c[2][2] ],
        ]
    }).flatten().collect_vec();

    let part2 = data.iter().filter(|t| {
        t[0] + t[1] > t[2] &&
        t[1] + t[2] > t[0] &&
        t[2] + t[0] > t[1]
    }).count();

    println!("{} part 2: {}", title, part2);
}

fn main() {
    run("demo", "5 10 25\n");
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
