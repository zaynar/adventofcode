use std::collections::{HashMap, HashSet, VecDeque};

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

struct Machine {
    state: intcode::State,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
    idle: bool,
}

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);

    const MACHINES: usize = 50;

    let mut machines = (0..MACHINES).map(|i| {
        let mut inputs = VecDeque::new();
        inputs.push_back(i as i64);
        Machine {
            state: initial.clone(),
            inputs,
            outputs: Vec::new(),
            idle: false,
        }
    }).collect_vec();

    let mut nat = None;
    let mut seen = HashSet::new();

    loop {
        for i in 0..MACHINES {
            let mut sent = None;

            let m = &mut machines[i];
            match m.state.runstate {
                RunState::Halted => (),
                RunState::Ready => m.state.step(),
                RunState::HasInput(_) => m.state.step(),
                RunState::NeedsInput => {
                    m.idle = m.inputs.is_empty();
                    m.state.runstate = RunState::HasInput(m.inputs.pop_front().unwrap_or(-1));
                },
                RunState::HasOutput(n) => {
                    m.idle = false;
                    m.outputs.push(n);
                    if m.outputs.len() == 3 {
                        let addr = m.outputs[0];
                        let x = m.outputs[1];
                        let y = m.outputs[2];
                        sent = Some((addr, x, y));
                        m.outputs.clear();
                    }
                    m.state.runstate = RunState::Ready;
                }
            }

            if let Some((addr, x, y)) = sent {
                // println!("Send {}->{}: {} {}", i, addr , x, y);

                if addr == 255 {
                    // println!("{} part 1: {} {}", title, x, y);
                    nat = Some((x, y));
                } else {
                    machines[addr as usize].inputs.push_back(x);
                    machines[addr as usize].inputs.push_back(y);
                }
            }
        }

        if machines.iter().all(|m| m.idle) {
            if let Some((x, y)) = nat {
                println!("Send NAT->0: {} {}", x, y);
                if !seen.insert(y) {
                    println!("{} part 2: {}", title, y);
                    return;
                }
                machines[0].inputs.push_back(x);
                machines[0].inputs.push_back(y);
                nat = None;
            }
        }

    }
}

fn main() {
    println!("====================================");
    run("input", &std::fs::read_to_string("23/input.txt").unwrap());
    // run2("input", &std::fs::read_to_string("21/input.txt").unwrap());
}
