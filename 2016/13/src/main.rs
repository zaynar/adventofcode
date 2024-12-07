use std::collections::{HashSet, VecDeque};

fn is_wall(x: i32, y: i32, seed: i32) -> bool {
    if x < 0 || y < 0 {
        return true;
    }

    let t = x*x + 3*x + 2*x*y + y + y*y;
    let t = t + seed;
    t.count_ones() % 2 == 1
}

fn run(title: &str, goal: (i32, i32), seed: i32) {
    for y in 0..7 {
        for x in 0..10 {
            print!("{}", if is_wall(x, y, seed) { "#" } else { "." });
        }
        println!();
    }

    let mut open = VecDeque::new();
    let mut closed = HashSet::new();
    open.push_back((1, 1, 0));

    while let Some((x, y, steps)) = open.pop_front() {
        if steps > 50 {
            println!("{} part 2: {}", title, closed.len());
            break;
        }

        if !closed.insert((x, y)) {
            continue;
        }

        if (x, y) == goal {
            println!("{} part 1: {}", title, steps);
            break;
        }
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if !is_wall(x + d.0, y + d.1, seed) {
                open.push_back((x + d.0, y + d.1, steps + 1));
            }
        }
    }
}

fn main() {
    run("demo", (7, 4), 10);
    run("input", (31, 39), 1358);
}
