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

fn part1(input: &Vec<Vec<char>>) {
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
    }

    println!("Answer 1: {}", max_goal);
}

fn part2(input: &Vec<Vec<char>>) {
    let w = input[0].len();
    let h = input.len();

    // We can simplify the graph to only include nodes for any cells with
    // a branching decision (>2 neighbours).
    // Find those nodes (and edges between them) by flood-filling the graph.
    // Then we can cheaply test every path through those nodes to find the longest

    let mut nodes: Vec<Coord> = Vec::new();

    // (from, to, len)
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();

    nodes.push(Coord { x: 1, y: 0 });
    nodes.push(Coord { x: w-2, y: h-1 });

    // (new cell, last node, len)
    let mut open: Vec<(Coord, usize, usize)> = Vec::new();
    open.push((Coord { x: 1, y: 1 }, 0, 1));

    let mut visited: HashSet<Coord> = HashSet::new();

    while let Some((c, pred, len)) = open.pop() {
        let g = input[c.y][c.x];
        // println!("{:?} {} {} {}", c, g, pred, len);
        if g == '#' {
            continue;
        }

        // If we've reached a previously-seen node,
        // define an edge (if it's not a self-edge) and stop
        if let Some(node) = nodes.iter().position(|&n| n == c) {
            println!("{}->{}", pred, node);
            if pred != node {
                edges.push((pred, node, len));
                edges.push((node, pred, len));
            }
            continue;
        }

        // If we've already explored this cell, stop
        if !visited.insert(c) {
            continue;
        }

        let neighbours = [
            input[c.y][c.x - 1] != '#',
            input[c.y][c.x + 1] != '#',
            c.y > 0 && input[c.y - 1][c.x] != '#',
            c.y < h-1 && input[c.y + 1][c.x] != '#',
        ];

        let (pred, len) = if neighbours.iter().filter(|n| **n).count() > 2 {
            // This is a new node
            let id = nodes.len();
            edges.push((pred, id, len));
            edges.push((id, pred, len));
            nodes.push(c);
            (id, 1)
        } else {
            (pred, len + 1)
        };

        if neighbours[0] {
            open.push((Coord { x: c.x - 1, y: c.y }, pred, len));
        }
        if neighbours[1] {
            open.push((Coord { x: c.x + 1, y: c.y }, pred, len));
        }
        if neighbours[2] {
            open.push((Coord { x: c.x, y: c.y - 1 }, pred, len));
        }
        if neighbours[3] {
            open.push((Coord { x: c.x, y: c.y + 1 }, pred, len));
        }
    }

    println!("{:?}", nodes);
    println!("{:?}", edges);

    // for edge in edges {
    //     println!("n{} -> n{} [label={}];", edge.0, edge.1, edge.2);
    // }

    let mut max_len = 0;
    let mut paths: Vec< (Vec<usize>, usize) > = Vec::new();
    paths.push((vec![0], 0));
    while let Some((p, len)) = paths.pop() {
        let i = *p.last().unwrap();
        if i == 1 {
            // println!("{} {:?}", len, p);
            sum = sum.max(len);
            continue;
        }
        let ns: Vec<_> = edges.iter().filter(|&e| e.0 == i && !p.contains(&e.1)).collect();
        // println!("{:?} + {:?}", p, ns);
        for n in ns {
            let mut np = p.clone();
            np.push(n.1);
            paths.push((np, len + n.2))
        }
    }
    println!("Answer 2: {}", max_len);
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("input").unwrap().lines().map(|line| line.chars().collect()).collect();
    part1(&input);
    part2(&input);
}
