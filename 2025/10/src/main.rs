// Part 1: 22 mins
// Part 1+2: 506 mins

// Rough overview:
//
// Convert the buttons and joltages into a system of linear equations (where the variables are the
// number of times you press each button). Put that in a matrix, and simplify with Gaussian elimination.
//
// If you end up with a diagonal matrix (plus some zero rows at the bottom), then you've found the
// unique solution, so add up all the variables and that's the answer.
//
// Otherwise, pick a column in the simplified matrix with lots of non-zero coefficients. That represents
// a variable j which hasn't been solved yet, so we need to just guess its value. Hopefully that will
// allow many more rows to be solved.
//
// So, for k in 0..300, add a new equation j = k to your system and (recursively) try to solve again.
// Pick the successful solution with the lowest cost (if any).
//
// If the recursive call doesn't get a unique answer and has to guess another variable, make sure it
// picks one that hasn't already been guessed.
//
// As an optimisation: Initially, only allow a single level of recursion. If that doesn't find a successful
// solution, try with 2 levels of recursion (i.e. guess the values of two variables). Repeat up to about 5 levels,
// since one of my inputs is really hard to solve, but most of them can be solved much quicker.
// (This is essentially doing BFS instead of DFS, but I implemented it with recursion so it gets a bit messy.)

use std::collections::VecDeque;

use itertools::Itertools;

use aocpath::Pathfinder;
use nalgebra as na;

use rayon::prelude::*;

fn solve(target: u32, buttons: &[u32]) -> i64 {
    type Node = u32;

    struct PathContext<'a> {
        buttons: &'a [u32],
        cost: i64,
    }

    impl<'a> aocpath::Callbacks<Node> for PathContext<'a> {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();
            for b in self.buttons {
                ret.push((1, *node ^ *b));
            }
            ret
        }

        fn found_path(&mut self, id: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            if *id == 0 {
                self.cost = cost;
                return Err(aocpath::PathError::Abort);
            }
            Ok(true)
        }
    }

    let mut ctx = PathContext { buttons, cost: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.bfs(&mut ctx, target);

    return ctx.cost;
}

fn solveeq(
    target: &[u32],
    buttons: &[u32],
    extras: &[(usize, u32)],
) -> (Option<i64>, na::DMatrix<f64>) {
    // Try to find a unique solution with Gaussian elimination

    let mut matrix = na::DMatrix::from_row_iterator(
        target.len() + extras.len(),
        buttons.len() + 1,
        target
            .iter()
            .enumerate()
            .flat_map(|(i, t)| {
                buttons
                    .iter()
                    .map(move |b| if b & (1 << i) != 0 { 1.0 } else { 0.0 })
                    .chain([*t as f64])
            })
            .chain(extras.iter().flat_map(|(b, c)| {
                (0..buttons.len())
                    .map(|i| if i == *b { 1.0 } else { 0.0 })
                    .chain([*c as f64])
            })),
    );

    // println!("--------\nextras={extras:?} t={target:?} b={buttons:?} {matrix}");

    let mut prevtop = 0;
    for col in 0..buttons.len() {
        // Normalise rows to start with 1.0
        for mut row in matrix.row_iter_mut() {
            if let Some(first) = row.iter().find(|n| **n != 0.0) {
                row /= *first;
            }
        }

        // Pick any row where the first non-zero value is in `col`
        if let Some(top) = matrix
            .row_iter()
            .position(|data| data.iter().take(col).all(|n| *n == 0.0) && data[col] != 0.0)
        {
            // Put in the appropriate spot to create our diagonal matrix
            matrix.swap_rows(top, prevtop);
            // println!("swap: {matrix}");

            // Subtract this row from all the others, so everything in `col` below
            // this row is 0.0, and hopefully everything above this row becomes more diagonal
            let toprow = matrix.row(prevtop).clone_owned();
            for (i, mut row) in matrix.row_iter_mut().enumerate() {
                if i != prevtop {
                    row -= toprow.clone() * row[col];

                    // Handle floating-point imprecision
                    row.apply(|n| {
                        if n.abs() < 1e-6 {
                            *n = 0.0
                        }
                    });
                }
            }
            // println!("subtract: {matrix}");

            prevtop += 1;
        }
    }

    // println!("final: {matrix}");

    // If this gave us a diagonal matrix, we can read off all the variables
    // and verify that it matches `target`

    let mut newtarget = target.to_vec();
    let mut startcost = 0;

    let mut presses = vec![0; buttons.len()];

    for row in matrix.row_iter() {
        if let Some(first) = row.iter().position(|n| *n != 0.0) {
            if first < buttons.len()
                && row
                    .iter()
                    .skip(first + 1)
                    .take(buttons.len() - first - 1)
                    .all(|n| *n == 0.0)
            {
                let c = *row.iter().last().unwrap();
                startcost += c.round() as i64;
                presses[first] += c.round() as u32;
                for i in 0..10 {
                    if buttons[first] & (1 << i) != 0 {
                        newtarget[i] -= c.round() as u32;
                    }
                }
            }
        }
    }

    // println!("{startcost} {target:?} {newtarget:?}");

    if newtarget.iter().all(|n| *n == 0) {
        println!("solved {startcost} e={extras:?} {target:?} {newtarget:?} presses={presses:?}");
        return (Some(startcost), matrix);
    }

    (None, matrix)
}

fn solve2(target: &[u32], buttons: &[u32]) -> i64 {
    let mut best: (u32, i64, Vec<u32>) = (1000, 0, target.to_vec());

    let numextras = 0;

    fn solve4(
        target: &[u32],
        buttons: &[u32],
        extras: &[(usize, u32)],
        maxextras: usize,
    ) -> Option<i64> {
        let (cost, matrix) = solveeq(target, buttons, extras);
        if let Some(cost) = cost {
            return Some(cost);
        }

        // As an optimisation, limit recursion
        if extras.len() >= maxextras {
            return None;
        }

        let mut newtarget = target.to_vec();
        for &(i, n) in extras {
            for j in 0..10 {
                if buttons[i] & (1 << j) != 0 {
                    if newtarget[j] < n {
                        return None;
                    }
                    newtarget[j] -= n;
                }
            }
        }
        let extrascost = extras.iter().map(|(_, n)| n).sum::<u32>();

        // Pick a semi-arbitrary column with plenty of non-zero values,
        // and guess what value it should have
        let worst = matrix
            .column_iter()
            .take(matrix.ncols() - 1)
            .enumerate()
            .filter(|(i, c)| !extras.iter().any(|(e, _)| e == i))
            .max_by_key(|(i, c)| c.magnitude_squared() as i64)
            .unwrap()
            .0;

        // println!("worst: {worst}");

        if let Some(cost) = (0..300)
            .rev()
            .filter_map(|k| {
                for j in 0..10 {
                    if buttons[worst] & (1 << j) != 0 && k > target[j] {
                        return None;
                    }
                }

                let mut e = extras.to_vec();
                e.push((worst, k));
                solve4(target, buttons, &e, maxextras)
            })
            .min()
        {
            return Some(cost);
        }

        None
    }

    for maxextras in 0..5 {
        if let Some(cost) = solve4(target, buttons, &[], maxextras) {
            return cost;
        }
    }

    println!(
        "NOT SOLVED t={target:?} b={buttons:?} {}",
        solveeq(target, buttons, &[]).1
    );

    // Dummy value to let us count how many weren't solved
    return 1000000;
}

fn run(title: &str, input: &str) {
    let data: Vec<(u32, Vec<u32>, Vec<u32>)> = input
        .lines()
        .map(|line| {
            let mut tokens: VecDeque<_> = line.split_ascii_whitespace().collect();
            let goal = tokens.pop_front().unwrap();
            let joltage = tokens.pop_back().unwrap();

            let mut goaln: u32 = 0;
            for c in goal
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .chars()
                .rev()
            {
                goaln = (goaln << 1)
                    + (match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!(),
                    });
            }

            let buttons = tokens
                .iter()
                .map(|t| {
                    let mut n: u32 = 0;
                    for c in t
                        .strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(",")
                    {
                        let c = c.parse::<u32>().unwrap();
                        assert!(c < 10);
                        n += 1 << c
                    }
                    n
                })
                .collect_vec();

            let jolts = joltage
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec();

            (goaln, buttons, jolts)
        })
        .collect();

    // println!("{:?}", data);

    let part1: i64 = data
        .iter()
        .map(|(goal, buttons, _)| solve(*goal, buttons))
        .sum();

    println!("{} part 1: {}", title, part1);

    let part2: i64 = data
        .par_iter()
        .map(|(_, buttons, jolts)| solve2(jolts, buttons))
        .sum();

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
