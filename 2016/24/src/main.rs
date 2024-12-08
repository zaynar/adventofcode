use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn flood(grid: &Vec<Vec<char>>, x0: i32, y0: i32) -> Vec<(i32, i32)> {
    let mut seen = HashSet::new();
    let mut open = VecDeque::new();
    open.push_back((x0, y0, 0));
    seen.insert((x0, y0));

    let mut ret = Vec::new();

    while let Some((x, y, d)) = open.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if seen.insert((x + dx, y + dy)) {
                match grid[(y + dy) as usize][(x + dx) as usize] {
                    c @ ('0'..='9') => {
                        ret.push((c.to_digit(10).unwrap() as i32, d + 1));
                        open.push_back((x + dx, y + dy, d + 1));
                    }
                    '.' => {
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

// #[derive(PartialEq, Eq, Clone, Hash, Debug)]
// struct State2 {
//     pos: [char; 4],
//     keys: Vec<char>,
//     steps: i32,
// }

// impl PartialOrd for State2 {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for State2 {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         other
//             .steps
//             .cmp(&self.steps)
//             .then(self.keys.cmp(&other.keys))
//             .then(self.pos.cmp(&other.pos))
//     }
// }

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
                src @ ('0'..='9') => {
                    for (dst, d) in flood(&grid, x as i32, y as i32) {
                        edges
                            .entry(src.to_digit(10).unwrap() as i32)
                            .or_insert_with(|| HashSet::new())
                            .insert((dst, d));
                    }
                }
                _ => (),
            }
        }
    }

    let num_keys = edges.keys().count();

    println!("{:?}", edges);
    println!("{} keys", num_keys);

    let mut shortest = i32::MAX;
    for p in (1..num_keys as i32).permutations(num_keys - 1) {
        let mut prev = 0;
        let mut dist = 0;
        for n in &p {
            dist += edges[&prev].iter().find_map(|e| if e.0 == *n { Some(e.1) } else { None }).unwrap();
            prev = *n;
        }

        // Part 2:
        dist += edges[&prev].iter().find_map(|e| if e.0 == 0 { Some(e.1) } else { None }).unwrap();

        // println!("{} {:?}", dist, p);
        shortest = shortest.min(dist);
    }

    println!("{} part N: {}", title, shortest);
}

const INPUT_DEMO: &str = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("24/input.txt").unwrap());
}
