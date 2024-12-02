fn fuel(n: i32) -> i32 {
    let f = (n / 3) - 2;
    if f < 0 {
        return 0;
    }
    f + fuel(f)
}

fn run(title: &str, input: &str) {
    let data: Vec<i32> = input
        .lines()
        .map(|n| str::parse(n).unwrap())
        .collect();

    println!(
        "{} part 1: {}",
        title,
        data.iter().map(|n| (n / 3) - 2).sum::<i32>()
    );

    println!(
        "{} part 2: {}",
        title,
        data.iter().map(|n| fuel(*n)).sum::<i32>()
    );
}

fn main() {
    run("demo", "1969");
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
