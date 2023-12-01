use std::{fs::File, io::{BufReader, BufRead}};

fn part1() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let first = line.chars().find(|c| char::is_ascii_digit(&c)).unwrap().to_digit(10).unwrap();
        let last = line.chars().rfind(|c| char::is_ascii_digit(&c)).unwrap().to_digit(10).unwrap();
        sum += first * 10 + last;
    }

    println!("{sum}");
}

fn parse_number(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    }

    let c = s.chars().next().unwrap();
    if char::is_ascii_digit(&c) {
        return Some(c.to_digit(10).unwrap());
    }

    if s.starts_with("one") { return Some(1); }
    if s.starts_with("two") { return Some(2); }
    if s.starts_with("three") { return Some(3); }
    if s.starts_with("four") { return Some(4); }
    if s.starts_with("five") { return Some(5); }
    if s.starts_with("six") { return Some(6); }
    if s.starts_with("seven") { return Some(7); }
    if s.starts_with("eight") { return Some(8); }
    if s.starts_with("nine") { return Some(9); }

    None
}

fn part2() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut first = None;
        let mut last = None;
        for i in 0..line.len() {
            if let Some(m) = parse_number(&line[i..]) {
                if first == None {
                    first = Some(m);
                }
                last = Some(m);
            }
        }

        println!("<< {line} >> {first:?} {last:?}");
        sum += first.unwrap() * 10 + last.unwrap();
    }

    println!("{sum}");
}

fn main() {
    part1();
    part2();
}
