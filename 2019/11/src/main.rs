use std::collections::HashMap;

use intcode::RunState;

mod intcode;

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);
    let mut state = initial.clone();

    let mut painted: HashMap<(i64, i64), i64> = HashMap::new();
    let mut dir: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut i = 0;

    'OUTER: loop {
        match state.runstate {
            RunState::Halted => break 'OUTER,
            RunState::Ready => state.step(),
            RunState::HasInput(_) => state.step(),
            RunState::NeedsInput => {
                state.runstate = RunState::HasInput(painted.get(&(x, y)).copied().unwrap_or(0));
            }
            RunState::HasOutput(n) => {
                if i % 2 == 0 {
                    painted.insert((x, y), n);
                } else {
                    dir = (dir + if n == 1 { 1 } else { -1 } + 4) % 4;
                    (x, y) = match dir {
                        0 => (x, y - 1),
                        1 => (x + 1, y),
                        2 => (x, y + 1),
                        3 => (x - 1, y),
                        _ => panic!(),
                    };
                }
                i += 1;

                state.runstate = RunState::Ready;
            }
        }
    }

    println!("{} part 1: {}", title, painted.values().len());
}

fn run2(title: &str, input: &str) {
    let initial = intcode::load(input);
    let mut state = initial.clone();

    let mut painted: HashMap<(i64, i64), i64> = HashMap::new();
    let mut dir: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut i = 0;

    painted.insert((0, 0), 1);

    'OUTER: loop {
        match state.runstate {
            RunState::Halted => break 'OUTER,
            RunState::Ready => state.step(),
            RunState::HasInput(_) => state.step(),
            RunState::NeedsInput => {
                state.runstate = RunState::HasInput(painted.get(&(x, y)).copied().unwrap_or(0));
            }
            RunState::HasOutput(n) => {
                if i % 2 == 0 {
                    painted.insert((x, y), n);
                } else {
                    dir = (dir + if n == 1 { 1 } else { -1 } + 4) % 4;
                    (x, y) = match dir {
                        0 => (x, y - 1),
                        1 => (x + 1, y),
                        2 => (x, y + 1),
                        3 => (x - 1, y),
                        _ => panic!(),
                    };
                }
                i += 1;

                state.runstate = RunState::Ready;
            }
        }
    }

    for y in (painted.keys().map(|&(x, y)| y).min().unwrap())..=(painted.keys().map(|&(x, y)| y).max().unwrap()) {
        for x in (painted.keys().map(|&(x, y)| x).min().unwrap())..=(painted.keys().map(|&(x, y)| x).max().unwrap()) {
            print!("{}", if painted.get(&(x, y)).copied().unwrap_or(0) == 0 { '.' } else { '#' } );
        }
        println!();
    }

    println!("{} part 2: {}", title, painted.values().len());
}

fn main() {
    run("input", &std::fs::read_to_string("11/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("11/input.txt").unwrap());
}
