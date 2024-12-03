use std::collections::HashMap;

use intcode::RunState;
use itertools::Itertools;

mod intcode;

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);
    let mut state = initial.clone();

    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();

    let mut output = Vec::new();

    'OUTER: loop {
        match state.runstate {
            RunState::Halted => break 'OUTER,
            RunState::Ready => state.step(),
            RunState::HasInput(_) => state.step(),
            RunState::NeedsInput => panic!(),
            RunState::HasOutput(n) => {
                output.push(n);
                if output.len() == 3 {
                    screen.insert((output[0], output[1]), output[2]);
                    output.clear();
                }
                state.runstate = RunState::Ready;
            }
        }
    }

    println!("{} part 1: {}", title, screen.values().filter(|&&c| c == 2).count());
}

fn run2(title: &str, input: &str) {
    let initial = intcode::load(input);
    let mut state = initial.clone();
    // state.verbose = true;
    state.set(0, 2);

    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();

    let mut output = Vec::new();

    let w = 42;
    let h = 20;

    let mut score = 0;

    'OUTER: loop {
        match state.runstate {
            RunState::Halted => break 'OUTER,
            RunState::Ready => state.step(),
            RunState::HasInput(_) => state.step(),
            RunState::NeedsInput => {
                let ball = screen.iter().filter(|(&(x, y), &c)| c == 4).next().unwrap();
                let bat = screen.iter().filter(|(&(x, y), &c)| c == 3).next().unwrap();

                if false {
                    for y in 0..h {
                        for x in 0..w {
                            print!("{}", match screen.get(&(x, y)).copied().unwrap_or(0) {
                                0 => ' ',
                                1 => '#',
                                2 => '=',
                                3 => '_',
                                4 => 'o',
                                _ => '?',
                            });
                        }
                        println!();
                    }
                    println!("ball={:?} bat={:?}", ball, bat);
                    println!();
                }

                state.runstate = RunState::HasInput(ball.0.0.cmp(&bat.0.0) as i64);
            }
            RunState::HasOutput(n) => {
                output.push(n);
                if output.len() == 3 {
                    if output[0..=1] == [-1, 0] {
                        // println!("SCORE: {}", output[2]);
                        score = output[2];
                    } else {
                        screen.insert((output[0], output[1]), output[2]);
                    }
                    output.clear();
                }
                state.runstate = RunState::Ready;
            }
        }
    }

    println!("{} part 2: {}", title, score);
}

fn main() {
    run("input", &std::fs::read_to_string("13/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("13/input.txt").unwrap());
}
