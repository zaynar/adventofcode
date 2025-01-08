// Part 1: 10 mins
// Part 1+2: 14 mins

use std::collections::HashSet;

fn run(title: &str, input: &str) {

    let mut t: (i32, i32) = (0, 0);
    let mut h = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(t);

    for line in input.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let n: usize = n.parse().unwrap();

        let dir = match dir {
            "D" => (0, 1),
            "U" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!(),
        };

        for i in 0..n {
            // println!("h={:?} t={:?} d={:?}", h, t, dir);
            h = (h.0 + dir.0, h.1 + dir.1);
            if t.0.abs_diff(h.0) > 1 && t.1 == h.1 {
                t.0 += (h.0 - t.0).signum();
            } else if t.1.abs_diff(h.1) > 1 && t.0 == h.0 {
                t.1 += (h.1 - t.1).signum();
            } else if t.0.abs_diff(h.0) + t.1.abs_diff(h.1) > 2 {
                t.0 += (h.0 - t.0).signum();
                t.1 += (h.1 - t.1).signum();
            }

            assert!(t.0.abs_diff(h.0) < 2);
            assert!(t.1.abs_diff(h.1) < 2);

            visited.insert(t);
        }
    }

    println!("{} part 1: {}", title, visited.len());
}

fn run2(title: &str, input: &str) {

    let mut knots: [(i32, i32); 10] = [(0, 0); 10];

    let mut visited = HashSet::new();
    visited.insert(knots.last().unwrap().clone());

    for line in input.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let n: usize = n.parse().unwrap();

        let dir = match dir {
            "D" => (0, 1),
            "U" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!(),
        };

        for i in 0..n {
            // println!("h={:?} t={:?} d={:?}", h, t, dir);
            let nh = (knots[0].0 + dir.0, knots[0].1 + dir.1);
            knots[0] = nh;

            for k in 1..knots.len() {
                let h = knots[k - 1];
                let t = &mut knots[k];
                if t.0.abs_diff(h.0) > 1 && t.1 == h.1 {
                    t.0 += (h.0 - t.0).signum();
                } else if t.1.abs_diff(h.1) > 1 && t.0 == h.0 {
                    t.1 += (h.1 - t.1).signum();
                } else if t.0.abs_diff(h.0) + t.1.abs_diff(h.1) > 2 {
                    t.0 += (h.0 - t.0).signum();
                    t.1 += (h.1 - t.1).signum();
                }

                assert!(t.0.abs_diff(h.0) < 2);
                assert!(t.1.abs_diff(h.1) < 2);
            }

            visited.insert(knots.last().unwrap().clone());
        }
    }

    println!("{} part 2: {}", title, visited.len());
}

const INPUT_DEMO: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

const INPUT_DEMO2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("09/input.txt").unwrap());
    run2("demo2", INPUT_DEMO2);
    run2("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
