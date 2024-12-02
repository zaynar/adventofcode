fn safe(levels: &Vec<i32>) -> bool {
    let gaps = levels.windows(2).map(|w| w[1] - w[0]);
    let min = gaps.clone().min().unwrap();
    let max = gaps.clone().max().unwrap();
    return (min >= 1 && max <= 3) || (min >= -3 && max <= -1);
}

fn safe2(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let mut levels = levels.clone();
        levels.remove(i);
        let gaps = levels.windows(2).map(|w| w[1] - w[0]);
        let min = gaps.clone().min().unwrap();
        let max = gaps.clone().max().unwrap();
        // println!("{} {} {}", levels.iter().join(","), min, max);
        if (min >= 1 && max <= 3) || (min >= -3 && max <= -1) {
            // println!("SAFE");
            return true;
        }
    }
    // println!("UNSAFE");
    false
}

fn run(title: &str, input: &str) {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| str::parse(n).unwrap())
                .collect()
        })
        .collect();


    println!(
        "{} part 1: {}",
        title,
        data.iter().filter(|levels| safe(levels)).count()
    );

    println!(
        "{} part 2: {}",
        title,
        data.iter().filter(|levels| safe2(levels)).count()
    );

}

const INPUT_DEMO: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
