use std::collections::{HashMap, VecDeque};

use intcode::RunState;
use itertools::Itertools;

mod intcode;

#[derive(Debug)]
enum Step {
    R,
    L,
    F(u32),
}

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);

    let mut grid = HashMap::new();

    {
        let mut state = initial.clone();

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        'OUTER: loop {
            match state.runstate {
                RunState::Halted => break 'OUTER,
                RunState::Ready => state.step(),
                RunState::HasInput(_) => state.step(),
                RunState::NeedsInput => {
                    panic!();
                },
                RunState::HasOutput(n) => {
                    let c = n as u8 as char;
                    print!("{}", c);
                    grid.insert((x,y), c);
                    if c == '\n' {
                        (x, y) = (0, y + 1);
                    } else {
                        x += 1;
                    }
                    state.runstate = RunState::Ready;
                }
            }
        }
    }

    let w = 45;
    let h = 33;

    let mut part1 = 0;
    for y in 0..h {
        for x in 0..w {
            if grid.get(&(x, y)) == Some(&'#') {
                let ns = [
                    (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)
                ].iter().filter(|xy| grid.get(xy) == Some(&'#')).count();
                if ns == 4 {
                    part1 += x * y;
                }
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    {
        let mut steps = Vec::new();
        let ((mut x, mut y), _) = grid.iter().filter(|(p, c)| **c == '^').next().unwrap();
        let (mut dx, mut dy) = (0, -1);

        loop {
            if grid.get(&(x + dx, y + dy)) == Some(&'#') {
                if let Some(Step::F(n)) = steps.last() {
                    *steps.last_mut().unwrap() = Step::F(n + 1);
                } else {
                    steps.push(Step::F(1));
                }
                x += dx;
                y += dy;
            } else {
                let (rdx, rdy) = (-dy, dx);
                let (ldx, ldy) = (dy, -dx);
                if grid.get(&(x + rdx, y + rdy)) == Some(&'#') {
                    steps.push(Step::R);
                    (dx, dy) = (rdx, rdy);
                } else if grid.get(&(x + ldx, y + ldy)) == Some(&'#') {
                    steps.push(Step::L);
                    (dx, dy) = (ldx, ldy);
                } else {
                    break;
                }
            }
        }

        println!("{:?}", steps);
    }

    // R,8,L,12,R,8,R,12,L,8,R,10,R,12,L,8,R,10,R,8,L,12,R,8,R,8,L,8,L,8,R,8,R,10,R,8,L,12,R,8,R,8,L,12,R,8,R,8,L,8,L,8,R,8,R,10,R,12,L,8,R,10,R,8,L,8,L,8,R,8,R,10
    //
    // Manually split the path into segments

    {
        let mut state = initial.clone();
        state.set(0, 2);

        let input = "A,B,B,A,C,A,A,C,B,C
R,8,L,12,R,8
R,12,L,8,R,10
R,8,L,8,L,8,R,8,R,10
n
";
        let mut input_idx = 0;

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        'OUTER: loop {
            match state.runstate {
                RunState::Halted => break 'OUTER,
                RunState::Ready => state.step(),
                RunState::HasInput(_) => state.step(),
                RunState::NeedsInput => {
                    state.runstate = RunState::HasInput(input.as_bytes()[input_idx] as i64);
                    input_idx += 1;
                },
                RunState::HasOutput(n) => {
                    if n < 128 {
                        let c = n as u8 as char;
                        print!("{}", c);
                        grid.insert((x,y), c);
                        if c == '\n' {
                            (x, y) = (0, y + 1);
                        } else {
                            x += 1;
                        }
                    } else {
                        println!("[{}]", n);
                    }

                    state.runstate = RunState::Ready;
                }
            }
        }
    }
}


fn main() {
    run("input", &std::fs::read_to_string("17/input.txt").unwrap());
}
