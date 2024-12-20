// Part 1: 8 mins
// Part 1+2: 40 mins

use std::{cmp::Ordering, collections::{BinaryHeap, VecDeque}, ops::Range};

use itertools::Itertools;

fn dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) +
    a.1.abs_diff(b.1) +
    a.2.abs_diff(b.2)) as i64
}

fn run(title: &str, input: &str) {
    let data: Vec<((i64, i64, i64), i64)> = input
        .lines()
        .map(|line| {
            let (pos, r) = line.split_once(", ").unwrap();
            let pos = pos.strip_prefix("pos=<").unwrap().strip_suffix(">").unwrap();
            let r = r.strip_prefix("r=").unwrap().parse().unwrap();

            let pos = pos.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap();

            (pos, r)

        })
        .collect();

    // println!("{:?}", data);

    let mut best = (0, 0);
    for i in 0..data.len() {
        let mut in_range = 0;
        for j in 0..data.len() {
            // if i == j {
            //     continue;
            // }
            if dist(data[i].0, data[j].0) <= data[i].1 {
                in_range += 1;
            }
        }
        best = best.max((data[i].1, in_range))
    }

    println!("{} part 1: {:?}", title, best);
}

fn count_approx(range: (Range<i64>, Range<i64>, Range<i64>), data: &Vec<((i64, i64, i64), i64)>) -> usize {
    let mut ret = 0;
    for ((x, y, z), r) in data {
        if x + r < range.0.start ||
           x - r >= range.0.end ||
           y + r < range.1.start ||
           y - r >= range.1.end ||
           z + r < range.2.start ||
           z - r >= range.2.end {
            continue;
        }

        let mut ok = false;
        for rx in [range.0.start, range.0.end - 1] {
            for ry in [range.1.start, range.1.end - 1] {
                for rz in [range.2.start, range.2.end - 1] {
                    if range.0.contains(x) ||
                    range.1.contains(y) ||
                    range.2.contains(z) ||
                     dist((rx, ry, rz), (*x, *y, *z)) <= *r {
                        ok = true;
                    }
                }
            }
        }
        if !ok {
            continue;
        }

        ret += 1;
    }
    ret
}

fn count_exact(pt: (i64, i64, i64), data: &Vec<((i64, i64, i64), i64)>) -> usize {
    let mut ret = 0;
    for (pos, r) in data {
        if dist(pt, *pos) <= *r {
            ret += 1;
        }
    }
    ret
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    count: usize,
    range: (Range<i64>, Range<i64>, Range<i64>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count).then_with(||
            (self.range.0.start, self.range.0.end,
                self.range.1.start, self.range.1.end,
                self.range.2.start, self.range.2.end,
            ).cmp(&(other.range.0.start, other.range.0.end,
                other.range.1.start, other.range.1.end,
                other.range.2.start, other.range.2.end,
            ))
        )
    }
}


fn run2(title: &str, input: &str) {
    let data: Vec<((i64, i64, i64), i64)> = input
        .lines()
        .map(|line| {
            let (pos, r) = line.split_once(", ").unwrap();
            let pos = pos.strip_prefix("pos=<").unwrap().strip_suffix(">").unwrap();
            let r = r.strip_prefix("r=").unwrap().parse().unwrap();

            let pos = pos.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap();

            (pos, r)

        })
        .collect();

    // println!("{:?}", data.iter().map(|(p, r)| p.0).min());
    // println!("{:?}", data.iter().map(|(p, r)| p.1).min());
    // println!("{:?}", data.iter().map(|(p, r)| p.2).min());

    let range = -200_000_000 .. 200_000_000;

    let mut best = Vec::new();

    let mut bestn = 0;

    let mut open = BinaryHeap::new();
    open.push( Node { count: data.len(), range: (range.clone(), range.clone(), range.clone()) });
    while let Some(subdiv) = open.pop() {
        println!("{:?}", subdiv);

        let range = subdiv.range;
        let mid = (
            range.0.start + (range.0.end - range.0.start) / 2,
            range.1.start + (range.1.end - range.1.start) / 2,
            range.2.start + (range.2.end - range.2.start) / 2,
        );

        if mid == (range.0.start, range.1.start, range.2.start) {
            let count = count_exact(mid, &data);
            bestn = bestn.max(count);
            best.push((count, mid));
            continue;
        }

        for s0 in [range.0.start .. mid.0, mid.0 .. range.0.end] {
            for s1 in [range.1.start .. mid.1, mid.1 .. range.1.end] {
                for s2 in [range.2.start .. mid.2, mid.2 .. range.2.end] {
                    let count = count_approx((s0.clone(), s1.clone(), s2.clone()), &data);
                    if count > bestn {
                        open.push(Node { count, range: (s0.clone(), s1.clone(), s2.clone()) });
                    }
                }
            }
        }

    }

    best.sort();
    println!("{:#?}", best);
    println!("{} part 2: {}", title, dist((0, 0, 0), best.last().unwrap().1));
}

const INPUT_DEMO: &str = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
";

const INPUT_DEMO2: &str = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("23/input.txt").unwrap());
    run2("demo", INPUT_DEMO2);
    run2("input", &std::fs::read_to_string("23/input.txt").unwrap());
}
