fn run(title: &str, input: &str) {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| str::parse(n).unwrap())
                .collect()
        })
        .collect();

    println!("{} part 1: {}", title, "TODO");

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
