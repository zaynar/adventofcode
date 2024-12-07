use itertools::Itertools;

fn test1(r: i64, start: i64, rest: &[i64]) -> bool {
    if rest.is_empty() {
        return start == r;
    }

    return test1(r, start + rest[0], &rest[1..])
    || test1(r, start * rest[0], &rest[1..]);
}

fn test2(r: i64, start: i64, rest: &[i64]) -> bool {
    if rest.is_empty() {
        return start == r;
    }

    return test2(r, start + rest[0], &rest[1..])
    || test2(r, start * rest[0], &rest[1..])
    // || test2(r, (start.to_string() + &rest[0].to_string()).parse().unwrap(), &rest[1..]);
    || test2(r, start * 10i64.pow(rest[0].ilog10() + 1) + rest[0], &rest[1..]);
}

fn run(title: &str, input: &str) {
    let data: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|line| {
            let (r, ops) = line.split_once(": ").unwrap();
            (r.parse().unwrap(), ops.split_whitespace().map(|n| n.parse().unwrap()).collect())
        })
        .collect();

    let part1 = data.iter().filter(|(r, ops)|
        test1(*r, ops[0], &ops[1..])
    ).collect_vec();

    let part2 = data.iter().filter(|(r, ops)|
        test2(*r, ops[0], &ops[1..])
    ).collect_vec();

    // println!("{:?}", data);
    // println!("{:?}", part1);

    println!("{} part 1: {}", title, part1.iter().map(|(r, ops)| *r).sum::<i64>());

    println!("{} part 2: {}", title, part2.iter().map(|(r, ops)| *r).sum::<i64>());
}

const INPUT_DEMO: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
