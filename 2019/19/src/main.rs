use std::collections::{HashMap, VecDeque};

use intcode::RunState;
use itertools::Itertools;

mod intcode;


fn probe(state: &intcode::State, x: i64, y: i64) -> bool {
    let mut state = state.clone();
    let inputs = [x, y];
    let mut input_idx = 0;

    loop {
        match state.runstate {
            RunState::Halted => panic!(),
            RunState::Ready => state.step(),
            RunState::HasInput(_) => state.step(),
            RunState::NeedsInput => {
                state.runstate = RunState::HasInput(inputs[input_idx]);
                input_idx += 1;
            },
            RunState::HasOutput(n) => {
                return n == 1;
            }
        }
    }
}

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);

    let mut part1 = 0;
    for y in 0..50 {
        for x in 0..50 {

            let n = probe(&initial, x, y);

            print!("{}", if n { '#' } else { '.' });
            if n { part1 += 1; }
        }
        println!();
    }

    println!("{} part 1: {}", title, part1);
}

fn run2(title: &str, input: &str) {
    let initial = intcode::load(input);

    let mut x1s = Vec::new();

    let mut x0 = 0;
    let mut x1 = 0;
    for y in 0.. {
        while !probe(&initial, x0, y) {
            x0 += 1;
        }
        x1 = x1.max(x0);
        while probe(&initial, x1, y) {
            x1 += 1;
        }

        x1s.push(x1);

        let size = 100;

        if y >= size {
            println!("{} {}..{} (..{})", y, x0, x1, x1s[(y - size + 1) as usize]);
        }

        if y >= size && x1s[(y - size + 1) as usize] - x0 >= size {
            println!("{} part 2: {},{} = {}", title, x0, y - size + 1, x0 * 10000 + (y - size + 1));
            break;
        }
    }
}


fn main() {
    run("input", &std::fs::read_to_string("19/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
