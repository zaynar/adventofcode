use std::collections::{HashMap, VecDeque};

use intcode::RunState;
use itertools::Itertools;

mod intcode;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Unknown,
    Wall,
    Space,
    Oxygen,
}

struct Node {
    x: i64,
    y: i64,
    steps: i64,
    state: intcode::State,
}

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);

    let mut maze: HashMap<(i64, i64), Cell> = HashMap::new();

    let mut open = VecDeque::new();
    open.push_back(Node { x: 0, y: 0, steps: 0, state: initial.clone() });

    let mut oxygen = (0, 0);

    'NODE: while let Some(node) = open.pop_front() {
        // println!("OPEN: {}", open.len());

        if maze.get(&(node.x, node.y)).copied().unwrap_or(Cell::Unknown) != Cell::Unknown {
            continue;
        }

        let mut state = node.state.clone();
        'OUTER: loop {
            match state.runstate {
                RunState::Halted => panic!(),
                RunState::Ready => state.step(),
                RunState::HasInput(_) => state.step(),
                RunState::NeedsInput => {
                    // println!("{} {} needs input", node.x, node.y);

                    for (dir, x, y) in [
                        (1, node.x, node.y - 1),
                        (2, node.x, node.y + 1),
                        (3, node.x - 1, node.y),
                        (4, node.x + 1, node.y),
                    ] {
                        match maze.get(&(x, y)).copied().unwrap_or(Cell::Unknown) {
                            Cell::Unknown | Cell::Space | Cell::Oxygen => {
                                let mut new_state = state.clone();
                                new_state.runstate = RunState::HasInput(dir);
                                open.push_back(Node { x, y, steps: node.steps + 1, state: new_state });
                            }
                            Cell::Wall => ()
                        }
                    }
                    break 'OUTER;
                },
                RunState::HasOutput(n) => {
                    // println!("{} {} outputs {}", node.x, node.y, n);
                    match n {
                        0 => {
                            maze.insert((node.x, node.y), Cell::Wall);
                            continue 'NODE;
                        }
                        1 => {
                            maze.insert((node.x, node.y), Cell::Space);
                        }
                        2 => {
                            maze.insert((node.x, node.y), Cell::Oxygen);
                            oxygen = (node.x, node.y);
                            println!("{} part 1: {}", title, node.steps);
                            // return;
                        }
                        _ => panic!(),
                    }
                    state.runstate = RunState::Ready;
                }
            }
        }
    }

    let (xmin, xmax) = (maze.keys().map(|&(x, y)| x).min().unwrap(), maze.keys().map(|&(x, y)| x).max().unwrap());
    let (ymin, ymax) = (maze.keys().map(|&(x, y)| y).min().unwrap(), maze.keys().map(|&(x, y)| y).max().unwrap());
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            print!("{}", match maze.get(&(x, y)).copied().unwrap_or(Cell::Unknown) {
                Cell::Unknown => '?',
                Cell::Wall => '#',
                Cell::Space => ' ',
                Cell::Oxygen => 'O',
            });
        }
        println!();
    }

    for i in 0.. {
        let mut new_maze = maze.clone();

        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if maze.get(&(x, y)) == Some(&Cell::Space) {
                    if [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().any(|(dx, dy)| {
                        maze.get(&(x + dx, y + dy)) == Some(&Cell::Oxygen)
                    }) {
                        new_maze.insert((x, y), Cell::Oxygen);
                    };
                }
            }
        }

        if new_maze == maze {
            println!("{} part 2: {}", title, i);
            break;
        }

        maze = new_maze;
    }

}


fn main() {
    run("input", &std::fs::read_to_string("15/input.txt").unwrap());
}
