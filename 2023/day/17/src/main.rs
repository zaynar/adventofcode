use std::{fs, collections::{HashMap, BinaryHeap}, cmp::Reverse};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Node {
    x: isize,
    y: isize,
    steps: u32,
    step_dir: (isize, isize),
}

fn neighbours(n: &Node, gw: isize, gh: isize) -> Vec<Node> {
    let mut ps = Vec::new();
    if n.x > 0 {
        ps.push((n.x-1, n.y));
    }
    if n.x < gw-1 {
        ps.push((n.x+1, n.y));
    }
    if n.y > 0 {
        ps.push((n.x, n.y-1));
    }
    if n.y < gh-1 {
        ps.push((n.x, n.y+1));
    }

    let mut ns = Vec::new();
    for (x, y) in ps {
        let d = (x - n.x, y - n.y);
        if d == n.step_dir {
            if n.steps + 1 < 4 {
                ns.push(Node { x, y, steps: n.steps + 1, step_dir: n.step_dir });
            } else {
                // too long, not allowed
            }
        } else if d.0 == -n.step_dir.0 && d.1 == -n.step_dir.1 {
            // reversing not allowed
        } else {
            ns.push(Node { x, y, steps: 1, step_dir: d });
        }
    }

    ns
}

fn main() {
    let grid: Vec<Vec<u32>> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let gw = grid[0].len() as isize;
    let gh = grid.len() as isize;

    let start = Node { x: 0, y: 0, steps: 0, step_dir: (0, 0) };

    // println!("{:?}", grid);

    let mut dist: HashMap<Node, u32> = HashMap::new();
    dist.insert(start.clone(), 0);

    // let open: Vec<Node> = Vec::new();
    // open.push(start);

    let mut Q: BinaryHeap<(Reverse<u32>, Node)> = BinaryHeap::new();
    Q.push((Reverse(0), start.clone()));

    while let Some((udist, unode)) = Q.pop() {
        // println!("{:?} {:?}", udist, unode);
        if unode.x == gw-1 && unode.y == gh-1 {
            println!("Answer: {}", udist.0);
            break;
        }

        if let Some(&d) = dist.get(&unode) {
            if udist.0 > d {
                continue;
            }
        }

        for n in neighbours(&unode, gw, gh) {
            let d = udist.0 + grid[n.y as usize][n.x as usize];
            if d < *dist.get(&n).unwrap_or(&u32::MAX) {
                Q.push((Reverse(d), n.clone()));
                dist.insert(n.clone(), d);
            }

        }
    }

}
