// Part 1: 10 mins
// Part 1+2: 34 mins

use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

use aocgrid::Grid;

#[derive(PartialEq, Eq, Debug)]
struct State {
    cost: usize,
    pos: (i32, i32),
    dir: (i32, i32),
    path: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct Seen {
    cost: usize,
    preds: HashSet<(i32, i32)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then((self.pos, self.dir).cmp(&(other.pos, other.dir)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run(title: &str, input: &str) {
    let grid = Grid::from(input);
    // grid.print();

    let start = grid.find(&'S').unwrap();
    let end = grid.find(&'E').unwrap();

    let mut open = BinaryHeap::new();
    open.push(State { cost: 0, pos: start, dir: (1, 0), path: vec![start] });
    let mut seen: HashMap<((i32, i32), (i32, i32)), Seen> = HashMap::new();

    let mut best = None;

    while let Some(node) = open.pop() {
        if node.pos == end {
            // println!("end {:?}", node);
            if best.is_none() {
                println!("{} part 1: {}", title, node.cost);
                best = Some(node.cost);
            }
        }

        if let Some(b) = best {
            println!("abort {}", node.cost);
            if node.cost > b {
                break;
            }
        }

        // if !seen.insert((node.pos, node.dir)) {
        //     continue;
        // }
        if let Some(s) = seen.get(&(node.pos, node.dir)) {
            if s.cost < node.cost {
                continue;
            } else if node.cost < s.cost {
                let mut preds = HashSet::new();
                for p in &node.path {
                    preds.insert(*p);
                }
                seen.insert((node.pos, node.dir), Seen { cost: node.cost, preds });
            } else {
                for p in &node.path {
                    seen.get_mut(&(node.pos, node.dir)).unwrap().preds.insert(*p);
                }
            }
        } else {
            let mut preds = HashSet::new();
            for p in &node.path {
                preds.insert(*p);
            }
            seen.insert((node.pos, node.dir), Seen { cost: node.cost, preds });
        }

        open.push(State { cost: node.cost + 1000, pos: node.pos, dir: (-node.dir.1, node.dir.0), path: node.path.clone() });
        open.push(State { cost: node.cost + 1000, pos: node.pos, dir: (node.dir.1, -node.dir.0), path: node.path.clone() });
        let newpos = (node.pos.0 + node.dir.0, node.pos.1 + node.dir.1);
        if *grid.get(newpos.0, newpos.1) != '#' {
            let mut path = node.path.clone();
            path.push(newpos);
            open.push(State { cost: node.cost + 1, pos: newpos, dir: node.dir, path });
        }
    }

    let mut part2 = HashSet::new();
    for dir in [(1,0),(-1,0),(0,1),(0,-1)] {
        if let Some(s) = seen.get(&(end, dir)) {
            for p in &s.preds {
                part2.insert(*p);
            }
        }
    }

    let mut grid2 = grid.clone();
    for p in &part2 {
        grid2.set(p.0, p.1, 'O');
    }
    println!("{}", grid2);

    println!("{} part 2: {:?}", title, part2.len());
}

const INPUT_DEMO: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
