// Part 1: 10 mins
// Part 1+2: 11 mins

use fancy_regex::Regex;

fn run(title: &str, input: &str) {
    let data: Vec<(i64, i64)> = input
        .trim_ascii()
        .split(",")
        .map(|r| {
            let (a, b) = r.split_once("-").unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect();

    let re = Regex::new(r"^(\d+)\1$").unwrap();
    let re2 = Regex::new(r"^(\d+)\1+$").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for (a, b) in data {
        for n in a..=b {
            if re.is_match(&n.to_string()).unwrap() {
                part1 += n;
            }
            if re2.is_match(&n.to_string()).unwrap() {
                part2 += n;
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
