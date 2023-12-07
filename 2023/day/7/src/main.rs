use std::{cmp::Ordering, collections::HashMap, fs};

use itertools::Itertools;

#[derive(Debug)]
struct Line {
    ty: Type,
    cards: Vec<u32>,
    bid: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

fn compare(a: &Line, b: &Line) -> Ordering {
    a.ty.cmp(&b.ty).then(a.cards.cmp(&b.cards))
}

fn hand_type(cards: &Vec<u32>) -> Type {
    let mut counts = HashMap::new();
    for c in cards {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut cs: Vec<_> = counts.values().copied().collect();
    cs.sort();

    if cs == vec![5] {
        Type::Five
    } else if cs == vec![1, 4] {
        Type::Four
    } else if cs == vec![2, 3] {
        Type::Full
    } else if cs == vec![1, 1, 3] {
        Type::Three
    } else if cs == vec![1, 2, 2] {
        Type::Two
    } else if cs == vec![1, 1, 1, 2] {
        Type::One
    } else if cs == vec![1, 1, 1, 1, 1] {
        Type::High
    } else {
        unreachable!();
    }
}

fn best_hand_type(cards: &Vec<u32>) -> Type {
    cards
        .iter()
        .map(|c| match c {
            0 => (2..=13).collect_vec(),
            &x => vec![x],
        })
        .multi_cartesian_product()
        .map(|cs| hand_type(&cs))
        .max()
        .unwrap()
}

fn parse_card1(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn parse_card2(c: char) -> u32 {
    match c {
        'J' => 0,
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => unreachable!(),
    }
}

fn parse1(line: &str) -> Line {
    let cards = line[0..5].chars().map(parse_card1).collect();
    let bid = line[6..].parse().unwrap();
    Line {
        ty: hand_type(&cards),
        cards,
        bid,
    }
}

fn parse2(line: &str) -> Line {
    let cards = line[0..5].chars().map(parse_card2).collect();
    let bid = line[6..].parse().unwrap();
    Line {
        ty: best_hand_type(&cards),
        cards,
        bid,
    }
}

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let mut lines: Vec<_> = file.lines().map(parse1).collect();
    lines.sort_by(compare);
    let answer1: u32 = lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i as u32 + 1) * line.bid)
        .sum();
    println!("{}", answer1);

    let file = fs::read_to_string("input").unwrap();
    let mut lines: Vec<_> = file.lines().map(parse2).collect();
    // println!("{:?}", lines);
    lines.sort_by(compare);
    // println!("{:?}", lines);
    let answer2: u32 = lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i as u32 + 1) * line.bid)
        .sum();
    println!("{}", answer2);
}
