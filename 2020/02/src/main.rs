// Part 1: 3 mins
// Part 1+2: 5 mins

fn run(title: &str, input: &str) {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let (rule, pw) = line.split_once(": ").unwrap();
        let (range, ch) = rule.split_once(" ").unwrap();
        let (min, max) = range.split_once("-").unwrap();
        let min: usize = min.parse().unwrap();
        let max: usize = max.parse().unwrap();
        let ch = ch.chars().next().unwrap();

        let l = pw.chars().filter(|c| *c == ch).count();
        if min <= l && l <= max {
            part1 += 1;
        }

        if (pw.chars().nth(min-1).unwrap() == ch) != (pw.chars().nth(max-1).unwrap() == ch) {
            part2 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
