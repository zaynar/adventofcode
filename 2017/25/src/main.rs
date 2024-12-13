// Part 1: 8 mins

use std::collections::HashMap;

enum State {
    A, B, C, D, E, F
}
enum Dir {
    L,
    R,
}

fn main() {
    let mut steps = 12656374;
    let mut tape = vec![false; steps * 2];
    let mut cursor = steps;
    let mut state = State::A;
    for step in 0..steps {
        let v = tape[cursor];
        let (write, dir, new) = match state {
            State::A => if !v {
                (1, Dir::R, State::B)
            } else {
                (0, Dir::L, State::C)
            },
            State::B => if !v {
                (1, Dir::L, State::A)
            } else {
                (1, Dir::L, State::D)
            },
            State::C => if !v {
                (1, Dir::R, State::D)
            } else {
                (0, Dir::R, State::C)
            },
            State::D => if !v {
                (0, Dir::L, State::B)
            } else {
                (0, Dir::R, State::E)
            },
            State::E => if !v {
                (1, Dir::R, State::C)
            } else {
                (1, Dir::L, State::F)
            },
            State::F => if !v {
                (1, Dir::L, State::E)
            } else {
                (1, Dir::R, State::A)
            },
        };
        tape[cursor] = write == 1;
        cursor = match dir {
            Dir::L => cursor - 1,
            Dir::R => cursor + 1,
        };
        state = new;
    }
    println!("part 1: {}", tape.iter().filter(|b| **b).count());
}
