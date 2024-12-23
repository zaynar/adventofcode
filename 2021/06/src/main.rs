// Part 1: 5 mins
// Part 1+2: 7 mins

use std::collections::HashMap;

fn count(fish: u64, days: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if days == 0 {
        return 1;
    }

    if let Some(r) = cache.get(&(fish, days)) {
        return *r;
    }

    let r = if fish == 0 {
        count(6, days - 1, cache) + count(8, days - 1, cache)
    } else {
        count(fish - 1, days - 1, cache)
    };

    cache.insert((fish, days), r);
    r
}

fn run(title: &str, input: &str) {
    let data: Vec<u64> = input.trim().split(",").map(|n| n.parse().unwrap()).collect();

    // for d in 0..19 {
    //     println!("{} {}", d, data.iter().map(|f| count(*f, d)).sum::<u64>());
    // }

    let mut cache = HashMap::new();

    println!("{} part 1: {}", title, data.iter().map(|f| count(*f, 80, &mut cache)).sum::<u64>());

    println!("{} part 2: {}", title, data.iter().map(|f| count(*f, 256, &mut cache)).sum::<u64>());
}

const INPUT_DEMO: &str = "3,4,3,1,2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
