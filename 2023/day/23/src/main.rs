use std::{fs, collections::{HashSet, HashMap}};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn path_len(paths: &Vec<(Coord, usize)>, path: usize) -> usize {
    let mut p = path;
    let mut len = 0;
    while p != 0 {
        len += 1;
        p = paths[p].1;
    }
    len
}

fn path_contains(paths: &Vec<(Coord, usize)>, path: usize, c: Coord) -> bool {
    let mut p = path;
    while p != 0 {
        let n = paths[p];
        if n.0 == c {
            return true;
        }
        p = n.1;
    }
    false
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("input").unwrap().lines().map(|line| line.chars().collect()).collect();
    let w = input[0].len();
    let h = input.len();

    // New node to consider, path leading up to (not including) this node
    let mut open: Vec<(Coord, usize)> = Vec::new();
    open.push((Coord { x: 1, y: 0 }, 0));

    let mut paths: Vec<(Coord, usize)> = Vec::new();
    paths.push((Coord { x: 1, y: 0 }, 0));

    let mut max_goal = 0;
    while let Some((c, path)) = open.pop() {
        let g = input[c.y][c.x];
        if g == '#' {
            continue;
        }
        if path_contains(&paths, path, c) {
            // Looped
            continue;
        }
        if c == (Coord { x: w-2, y: h-1 }) {
            let len = path_len(&paths, path);
            println!("Goal: {:?} ({})", len, max_goal);
            max_goal = max_goal.max(len);
            continue;
        }

        let new_path = paths.len();
        paths.push((c, path));

        if false {
            if ['.', '<'].contains(&g) {
                open.push((Coord { x: c.x - 1, y: c.y }, new_path));
            }
            if ['.', '>'].contains(&g) {
                open.push((Coord { x: c.x + 1, y: c.y }, new_path));
            }
            if ['.', '^'].contains(&g) && c.y > 0 {
                open.push((Coord { x: c.x, y: c.y - 1 }, new_path));
            }
            if ['.', 'v'].contains(&g) {
                open.push((Coord { x: c.x, y: c.y + 1 }, new_path));
            }
        } else {
            // Part 2
            open.push((Coord { x: c.x - 1, y: c.y }, new_path));
            open.push((Coord { x: c.x + 1, y: c.y }, new_path));
            if c.y > 0 {
                open.push((Coord { x: c.x, y: c.y - 1 }, new_path));
            }
            open.push((Coord { x: c.x, y: c.y + 1 }, new_path));
        }
    }

    /* Part 2:
    Identify all nodes with >= 2 neighbours
    Get dist between adjacent nodes

    Now we only care about those nodes & loops between them
    */

    println!("Answer: {}", max_goal);
}
