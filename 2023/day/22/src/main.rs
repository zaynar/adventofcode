use std::{fs, collections::{HashSet, HashMap}};
use itertools::Either;

struct Grid {
    size: (usize, usize, usize),
    cells: Vec<usize>,
}

fn range(start: usize, end: usize) -> impl Iterator<Item=usize> {
    if start < end {
        Either::Left(start..=end)
    } else {
        Either::Right((end..=start).rev())
    }
}

const GROUND: usize = 8888;
const EMPTY: usize = 9999;

impl Grid {
    fn new() -> Self {
        let size = (10, 10, 400);
        let mut cells = vec![EMPTY; size.0 * size.1 * size.2];

        for y in 0..size.1 {
            for x in 0..size.0 {
                cells[x + y * size.0] = GROUND;
            }
        }

        Grid { size, cells }
    }

    fn set(&mut self, brick: &(Vec<usize>, Vec<usize>), id: usize) {
        for z in range(brick.0[2], brick.1[2]) {
            for y in range(brick.0[1], brick.1[1]) {
                for x in range(brick.0[0], brick.1[0]) {
                    // assert!(x < self.size.0 && y < self.size.1 && z < self.size.2);
                    let c = &mut self.cells[x + y * self.size.0 + z * self.size.0 * self.size.1];
                    if id != EMPTY && *c != EMPTY {
                        panic!();
                    }
                    *c = id;
                }
            }
        }
    }

    fn get(&mut self, brick: &(Vec<usize>, Vec<usize>)) -> Vec<usize> {
        let mut ret = Vec::new();
        for z in range(brick.0[2], brick.1[2]) {
            for y in range(brick.0[1], brick.1[1]) {
                for x in range(brick.0[0], brick.1[0]) {
                    ret.push(self.cells[x + y * self.size.0 + z * self.size.0 * self.size.1]);
                }
            }
        }
        ret
    }
}

fn main() {
    let mut bricks: Vec<_> = fs::read_to_string("input").unwrap().lines().map(|line| {
        let (a, b) = line.split_once("~").unwrap();
        let a: Vec<usize> = a.split(",").map(|n| n.parse().unwrap()).collect();
        let b: Vec<usize> = b.split(",").map(|n| n.parse().unwrap()).collect();
        assert!(
            (a[0] == b[0] && a[1] == b[1] && a[2] == b[2]) ||
            (a[0] != b[0] && a[1] == b[1] && a[2] == b[2]) ||
            (a[0] == b[0] && a[1] != b[1] && a[2] == b[2]) ||
            (a[0] == b[0] && a[1] == b[1] && a[2] != b[2])
        );
        assert!(a[2] > 0 && b[2] > 0);
        (a, b)
    }).collect();

    println!("{:?}", bricks);

    let mut grid = Grid::new();
    for (id, brick) in bricks.iter().enumerate() {
        grid.set(brick, id);
    }

    loop {
        print!(".");
        let mut progress = false;
        for (id, brick) in bricks.iter_mut().enumerate() {
            let mut below = brick.clone();
            below.0[2] -= 1;
            below.1[2] -= 1;
            let collide = grid.get(&below);
            if collide.iter().all(|&c| c == EMPTY || c == id) {
                grid.set(brick, EMPTY);
                *brick = below;
                grid.set(brick, id);
                progress = true;
            }
        }

        if !progress {
            break;
        }
    }

    println!("{:?}", bricks);

    let mut depends: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut reqd: HashSet<usize> = HashSet::new();
    for (id, brick) in bricks.iter_mut().enumerate() {
        let mut below = brick.clone();
        below.0[2] -= 1;
        below.1[2] -= 1;
        let collide = grid.get(&below);
        let mut s: HashSet<usize> = HashSet::from_iter(collide.iter().copied());
        s.remove(&EMPTY);
        // println!("{} supp by {:?}", id, s);
        if !s.contains(&GROUND) {
            s.remove(&GROUND);
            s.remove(&id);
            // println!("{} supp by {:?}", id, s);
            if s.len() == 1 {
                reqd.extend(&s);
            }
            depends.insert(id, s);
        }
    }

    // println!("{:?}", reqd);
    println!("Answer 1: {:?}", bricks.len() - reqd.len());

    println!("{:?}", depends);

    let mut sum2 = 0;
    for id in 0..bricks.len() {
        let mut removed: HashSet<usize> = HashSet::new();
        removed.insert(id);

        loop {
            let mut changed = false;
            for j in 0..bricks.len() {
                if let Some(d) = depends.get(&j) {
                    if !removed.contains(&j) && d.is_subset(&removed) {
                        removed.insert(j);
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        println!("{} {:?}", id, removed);
        sum2 += removed.len() - 1;
    }
    println!("Answer 2: {}", sum2);
}
