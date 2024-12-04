use std::collections::{HashMap, VecDeque};

use intcode::RunState;
use itertools::Itertools;

mod intcode;


fn probe(state: &intcode::State, input: &str) -> bool {
    let mut state = state.clone();

    let in_bytes = input.as_bytes();
    let mut idx = 0;

    loop {
        match state.runstate {
            RunState::Halted => return false,
            RunState::Ready => state.step(),
            RunState::HasInput(_) => state.step(),
            RunState::NeedsInput => {
                if idx >= in_bytes.len() {
                    panic!("end of input");
                }
                state.runstate = RunState::HasInput(in_bytes[idx] as i64);
                idx += 1;
            },
            RunState::HasOutput(n) => {
                if n < 128 {
                    print!("{}", n as u8 as char);
                } else {
                    println!("[{}]", n);
                }
                state.runstate = RunState::Ready;
            }
        }
    }
}

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);

    // Need to jump if !A
    // Safe to jump if D
    // Useful to jump if !B || !C

    // J = (!A || !B || !C) && D
    probe(&initial, "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
");
}

fn run2(title: &str, input: &str) {
    let initial = intcode::load(input);

    // Jump if D(4) and (E(5) or H(8)), so we can land then walk or jump again

probe(&initial, "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN
");
}

fn main() {
    println!("====================================");
    run("input", &std::fs::read_to_string("21/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("21/input.txt").unwrap());
}
