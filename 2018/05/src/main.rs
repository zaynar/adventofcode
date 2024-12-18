// Part 1: 4 mins
// Part 1+2: 7 mins

use itertools::Itertools;

fn len(data: &Vec<char>) -> usize {
    let mut s = data.clone();
    loop {
        let mut t = Vec::new();
        for c in &s {
            let opp = if c.is_ascii_lowercase() { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() };
            if t.last() == Some(&opp) {
                t.pop();
            } else {
                t.push(*c);
            }
        }
        if t == s {
            break;
        }
        s = t;
    }

    s.len()
}

fn run(title: &str, input: &str) {
    let data: Vec<char> = input.trim().chars().collect();

    println!("{} part 1: {}", title, len(&data));

    println!("{} part 2: {:?}", title, ('a'..='z').map(|c|
        (len(&data.iter().copied().filter(|x| x.to_ascii_lowercase() != c).collect_vec()), c)
    ).min());
}

const INPUT_DEMO: &str = "dabAcCaCBAcCcaDA";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
