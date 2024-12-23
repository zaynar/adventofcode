// Part 1: 6 mins
// Part 1+2: 7 mins

fn run(title: &str, input: &str) {
    let data: Vec<u64> = input.trim().split(",").map(|n| n.parse().unwrap()).collect();

    let mut cost = u64::MAX;
    for i in 0..2000 {
        cost = cost.min(data.iter().map(|n| n.abs_diff(i)).sum::<u64>());
    }

    let mut cost2 = u64::MAX;
    for i in 0..2000 {
        cost2 = cost2.min(data.iter().map(|n| {
            let m = n.abs_diff(i);
            m * (m + 1) / 2
        }).sum::<u64>());
    }

    println!("{} part 1: {}", title, cost);

    println!("{} part 2: {}", title, cost2);
}

const INPUT_DEMO: &str = "16,1,2,0,4,2,7,1,2,14
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
