use std::collections::{HashSet, VecDeque};

fn run(title: &str, input: &str) {
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut part1 = 0;
    let h = data.len() as isize;
    let w = data[0].len() as isize;
    for y in 0..h {
        for x in 0..w {
            if data[y as usize][x as usize] == 0 {

                // println!("start {} {}", x, y);

                let mut open = VecDeque::new();
                let mut seen = HashSet::new();
                open.push_back((x, y));

                while let Some((x, y)) = open.pop_back() {
                    if seen.insert((x, y)) {
                        if data[y as usize][x as usize] == 9 {
                            part1 += 1;
                        }
                    } else {
                        continue;
                    }

                    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let nx = x as isize + d.0;
                        let ny = y as isize + d.1;
                        if 0 <= nx && nx < w && 0 <= ny && ny < h {
                            if data[ny as usize][nx as usize] == data[y as usize][x as usize] + 1 {
                                open.push_back((nx, ny));
                            }
                        }
                    }

                }

            }
        }
    }

    let mut part2 = 0;
    for y in 0..h {
        for x in 0..w {
            let mut trails = HashSet::new();
            if data[y as usize][x as usize] == 0 {

                // println!("start {} {}", x, y);

                let mut open = VecDeque::new();
                let mut seen = HashSet::new();
                open.push_back((x, y, vec![]));

                while let Some((x, y, preds)) = open.pop_back() {
                    if seen.insert((x, y, preds.clone())) {
                        if data[y as usize][x as usize] == 9 {
                            trails.insert((x, y, preds.clone()));
                        }
                    } else {
                        continue;
                    }

                    let mut preds2 = preds.clone();
                    preds2.push((x, y));

                    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let nx = x as isize + d.0;
                        let ny = y as isize + d.1;
                        if 0 <= nx && nx < w && 0 <= ny && ny < h {
                            if data[ny as usize][nx as usize] == data[y as usize][x as usize] + 1 {
                                open.push_back((nx, ny, preds2.clone()));
                            }
                        }
                    }

                }

            }
            part2 += trails.len();
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
