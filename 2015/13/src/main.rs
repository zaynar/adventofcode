use std::collections::HashMap;

use itertools::Itertools;

peg::parser! {
    grammar input_parser() for str {
        rule num() -> i32
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule change() -> i32
            = "gain " n:num() { n }
            / "lose " n:num() { -n }

        rule ident() -> String
            = s:$(['A'..='Z' | 'a'..='z']+) { s.to_owned() }

        rule line() -> (String, i32, String)
            = a:ident() " would " n:change() " happiness units by sitting next to " b:ident() ".\n" { (a, n, b) }

        pub rule file() -> Vec<(String, i32, String)>
            = line()+
    }
}

fn run(title: &str, input: &str) {
    let data = input_parser::file(input).unwrap();

    println!("{:?}", data);

    let mut change = HashMap::new();
    for (a, n, b) in &data {
        change.insert((a, b), n);
    }

    let mut names = data.iter().map(|(a, n, b)| a).dedup().collect_vec();

    let mut max_score = 0;
    for mut order in names.iter().permutations(names.len()) {
        order.push(order.first().unwrap());
        let mut score = 0;
        for pair in order.windows(2) {
            score += change[&(*pair[0], *pair[1])];
            score += change[&(*pair[1], *pair[0])];
        }
        // println!("{:?} {}", order, score);
        max_score = max_score.max(score);
    }

    println!("{} part 1: {}", title, max_score);

    let me = "me".to_owned();
    names.push(&me);

    let mut max_score = 0;
    for mut order in names.iter().permutations(names.len()) {
        order.push(order.first().unwrap());
        let mut score = 0;
        for pair in order.windows(2) {
            score += **change.get(&(*pair[0], *pair[1])).unwrap_or(&&0);
            score += **change.get(&(*pair[1], *pair[0])).unwrap_or(&&0);
        }
        // println!("{:?} {}", order, score);
        max_score = max_score.max(score);
    }

    println!("{} part 2: {}", title, max_score);
}

const INPUT_DEMO: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("13/input.txt").unwrap());
}
