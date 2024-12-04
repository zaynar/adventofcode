use core::panic;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn flood(grid: &Vec<Vec<char>>, x0: i32, y0: i32) -> Vec<(char, i32)> {
    let mut seen = HashSet::new();
    let mut open = VecDeque::new();
    open.push_back((x0, y0, 0));
    seen.insert((x0, y0));

    let mut ret = Vec::new();

    while let Some((x, y, d)) = open.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if seen.insert((x + dx, y + dy)) {
                match grid[(y + dy) as usize][(x + dx) as usize] {
                    c @ ('1'..='4' | 'a'..='z' | 'A'..='Z') => {
                        ret.push((c, d + 1));
                    }
                    '@' | '.' => {
                        open.push_back((x + dx, y + dy, d + 1));
                    }
                    '#' => (),
                    c @ _ => panic!("Invalid char {}", c),
                }
            }
        }
    }

    ret
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct State {
    pos: char,
    keys: BTreeSet<char>,
    steps: i32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.cmp(&other.keys))
            .then(self.pos.cmp(&other.pos))
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct State2 {
    pos: [char; 4],
    keys: Vec<char>,
    steps: i32,
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.cmp(&other.keys))
            .then(self.pos.cmp(&other.pos))
    }
}

fn run(title: &str, input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let w = grid[0].len();
    let h = grid.len();

    let mut edges = HashMap::new();

    /*

    Calculate matrix of shortest paths between all nodes (@, a, A), treating all aA as walls

    Starting at @, BFS the graph (filtering out doors we don't have keys for), until got all keys

     */

    for y in 0..h {
        for x in 0..w {
            match grid[y][x] {
                src @ ('@' | 'a'..='z' | 'A'..='Z') => {
                    for (dst, d) in flood(&grid, x as i32, y as i32) {
                        edges
                            .entry(src)
                            .or_insert_with(|| HashSet::new())
                            .insert((dst, d));
                    }
                }
                _ => (),
            }
        }
    }

    let num_keys = edges.keys().filter(|c| ('a'..='z').contains(c)).count();

    println!("{:?}", edges);
    println!("{} keys", num_keys);

    let mut added = HashMap::new();
    let mut open = BinaryHeap::new();
    open.push(State {
        pos: '@',
        keys: BTreeSet::new(),
        steps: 0,
    });

    while let Some(state) = open.pop() {
        if state.keys.len() == num_keys {
            println!("{} part 1: {}", title, state.steps);
            break;
        }

        for (dst, d) in edges.get(&state.pos).unwrap() {
            if ('A'..='Z').contains(dst) && !state.keys.contains(&dst.to_ascii_lowercase()) {
                continue;
            }

            let mut new_keys = state.keys.clone();
            if ('a'..='z').contains(dst) {
                new_keys.insert(*dst);
            }

            // if new_keys.len() == num_keys {
            //     println!("{} part 1: {}", title, state.steps);
            // }

            let new_state = State {
                pos: *dst,
                keys: new_keys,
                steps: state.steps + d,
            };

            match added.get(&(new_state.pos, new_state.keys.clone())) {
                Some(n) if *n <= new_state.steps => {
                    // prune
                }
                _ => {
                    added.insert((new_state.pos, new_state.keys.clone()), new_state.steps);
                    open.push(new_state);
                }
            }
        }
    }
}

fn run2(title: &str, input: &str) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let w = grid[0].len();
    let h = grid.len();

    let mut edges = HashMap::new();

    /*
    Same as before, but now the state contains 4 bot positions, and we can move any of them
     */

    {
        for y in 0..h {
            for x in 0..w {
                if grid[y][x] == '@' {
                    grid[y-1][x-1] = '1';
                    grid[y-1][x+0] = '#';
                    grid[y-1][x+1] = '2';
                    grid[y+0][x-1] = '#';
                    grid[y+0][x+0] = '#';
                    grid[y+0][x+1] = '#';
                    grid[y+1][x-1] = '3';
                    grid[y+1][x+0] = '#';
                    grid[y+1][x+1] = '4';
                }
            }
        }
    }

    // for line in &grid {
    //     println!("{}", line.iter().join(""));
    // }

    for y in 0..h {
        for x in 0..w {
            match grid[y][x] {
                src @ ('@' | '1'..='4' | 'a'..='z' | 'A'..='Z') => {
                    // println!("# {} {:?}", src, flood(&grid, x as i32, y as i32));
                    for (dst, d) in flood(&grid, x as i32, y as i32) {
                        edges
                            .entry(src)
                            .or_insert_with(|| HashSet::new())
                            .insert((dst, d));
                    }
                }
                _ => (),
            }
        }
    }

    let num_keys = edges.keys().filter(|c| ('a'..='z').contains(c)).count();

    println!("{:?}", edges);
    println!("{} keys", num_keys);

    let mut added = HashMap::new();
    let mut open = BinaryHeap::new();
    open.push(State2 {
        pos: ['1', '2', '3', '4'],
        keys: Vec::new(),
        steps: 0,
    });

    while let Some(state) = open.pop() {
        // println!("{:?}", state);

        if state.keys.len() == num_keys {
            println!("{} part 2: {}", title, state.steps);
            break;
        }

        for bot in 0..4 {
            if let Some(edge_set) = edges.get(&state.pos[bot]) {
                // println!(" {} {:?}", bot, edge_set);
                for (dst, d) in edge_set {
                    if ('A'..='Z').contains(dst) && !state.keys.contains(&dst.to_ascii_lowercase()) {
                        continue;
                    }

                    let mut new_keys = state.keys.clone();
                    if ('a'..='z').contains(dst) && !new_keys.contains(dst) {
                        new_keys.push(*dst);
                        new_keys.sort();
                    }

                    let mut new_pos = state.pos.clone();
                    new_pos[bot] = *dst;
                    let new_state = State2 {
                        pos: new_pos,
                        keys: new_keys,
                        steps: state.steps + d,
                    };

                    match added.get(&(new_state.pos, new_state.keys.clone())) {
                        Some(n) if *n <= new_state.steps => {
                            // prune
                        }
                        _ => {
                            added.insert((new_state.pos, new_state.keys.clone()), new_state.steps);
                            open.push(new_state);
                        }
                    }
                }
            }
        }
    }

    println!("@ {}", added.len());
    std::mem::forget(added);
}

const INPUT_DEMO: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
";

const INPUT_DEMO2: &str = "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap());
    run2("demo 2", INPUT_DEMO2);
    run2("input", &std::fs::read_to_string("18/input.txt").unwrap());
}
