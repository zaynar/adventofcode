// Part 1: 5 mins
// Part 1+2: 16 mins

use std::collections::{HashSet, VecDeque};

fn run(title: &str, input: &str) {
    let (p1, p2) = input.strip_prefix("Player 1:\n").unwrap().split_once("\nPlayer 2:\n").unwrap();
    let mut p1: VecDeque<usize> = p1.lines().map(|n| n.parse().unwrap()).collect();
    let mut p2: VecDeque<usize> = p2.lines().map(|n| n.parse().unwrap()).collect();

    // println!("{:?} {:?}", p1, p2);

    while !p1.is_empty() && !p2.is_empty() {
        let a = p1.pop_front().unwrap();
        let b = p2.pop_front().unwrap();
        if a > b {
            p1.push_back(a);
            p1.push_back(b);
        } else if b > a {
            p2.push_back(b);
            p2.push_back(a);
        }
        // println!("{:?} {:?}", p1, p2);
    }

    println!("{} part 1: {}", title, p1.iter().rev().enumerate().map(|(i, n)| (i + 1) * n).sum::<usize>());
    println!("{} part 1: {}", title, p2.iter().rev().enumerate().map(|(i, n)| (i + 1) * n).sum::<usize>());
}

fn combat(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (u32, VecDeque<usize>) {

    let mut seen = HashSet::new();

    loop {
        if p1.is_empty() {
            return (2, p2);
        }
        if p2.is_empty() {
            return (1, p1);
        }

        if !seen.insert((p1.clone(), p2.clone())) {
            return (1, p1);
        }

        let a = p1.pop_front().unwrap();
        let b = p2.pop_front().unwrap();

        let winner;
        if a <= p1.len() && b <= p2.len() {
            let mut t1 = p1.clone();
            let mut t2 = p2.clone();
            t1.resize(a, 0);
            t2.resize(b, 0);
            (winner, _) = combat(t1, t2);
        } else {
            if a > b {
                winner = 1;
            } else {
                winner = 2;
            }
        }

        if winner == 1 {
            p1.push_back(a);
            p1.push_back(b);
        } else {
            p2.push_back(b);
            p2.push_back(a);
        }
    }

}

fn run2(title: &str, input: &str) {
    let (p1, p2) = input.strip_prefix("Player 1:\n").unwrap().split_once("\nPlayer 2:\n").unwrap();
    let mut p1: VecDeque<usize> = p1.lines().map(|n| n.parse().unwrap()).collect();
    let mut p2: VecDeque<usize> = p2.lines().map(|n| n.parse().unwrap()).collect();

    // println!("{:?} {:?}", p1, p2);

    // If hands seen before, P1 wins

    // Both draw top card
    // If both have >= a,b cards left:
    //    Copy next a,b cards to new decks
    //    Find the winner
    // Otherwise higher value wins

    let winner = combat(p1, p2);

    println!("{:?}", winner);
    println!("{} part 2: {}", title, winner.1.iter().rev().enumerate().map(|(i, n)| (i + 1) * n).sum::<usize>());
}

const INPUT_DEMO: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("22/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("22/input.txt").unwrap());
}
