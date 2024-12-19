// Part 1: 47 mins
// Part 2: 52 mins

use std::collections::{HashSet, VecDeque};

use aocgrid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Unit {
    x: i32,
    y: i32,
    ty: char,
    hp: i32,
}

fn enemy(ty: char) -> char {
    match ty {
        'E' => 'G',
        'G' => 'E',
        _ => panic!()
    }
}

fn run(title: &str, input: &str) {
    let mut grid = Grid::from(input);

    // Iterate over all units y,x
    // Identify targets (enemy units)
    // Identify open squares adjacent to target
    // If adj to target, do not move
    // Else if no open squares, end turn
    // Else find closest open square, tie-break on y,x
    // Move along shortest path; tie-break on y,x
    //
    // Attack:
    //  Find all adj enemies
    //  Select one with fewest hp, tie-break y,x
    //  hp -= attackpower
    //  (200hp, 3ap)

    let mut units = Vec::new();
    grid.for_each(|x, y, c| {
        if *c == 'E' || *c == 'G' {
            units.push(Unit { x, y, ty: *c, hp: 200 });
        }
    });

    const VERBOSE: bool = false;

    for turn in 0.. {

        units.sort_by_key(|u| (u.y, u.x));

        for i in 0..units.len() {
            let mut unit = units[i].clone();

            if unit.hp <= 0 {
                continue;
            }

            if !units.iter().any(|t| t.hp > 0 && t.ty == enemy(unit.ty)) {
                println!("combat complete: {}, part1={}", turn, turn * (
                    units.iter().map(|u| u.hp.max(0)).sum::<i32>()
                ));
                return;
            }


            let in_range = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().any(|(dx, dy)| *grid.get(unit.x + dx, unit.y + dy) == enemy(unit.ty));
            if VERBOSE { println!("{} in range: {}", i, in_range); }

            if !in_range {
                let mut targets = Vec::new();

                {
                    let mut open = VecDeque::new();
                    open.push_back((unit.x, unit.y, 0));
                    let mut seen = HashSet::new();

                    while let Some((x, y, d)) = open.pop_front() {
                        if !seen.insert((x, y)) {
                            continue;
                        }

                        let mut is_target = false;
                        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = x + dx;
                            let ny = y + dy;

                            if *grid.get(nx, ny) == '.' {
                                open.push_back((nx, ny, d + 1));
                            } else if *grid.get(nx, ny) == enemy(unit.ty) {
                                is_target = true;
                            }
                        }
                        if is_target {
                            targets.push((d, y, x));
                        }
                    }
                }

                targets.sort();
                if let Some(target) = targets.first() {
                    let (_, ty, tx) = *target;
                    if VERBOSE { println!("{} target: {:?}", i, target); }

                    let mut dists = Grid::new_empty(grid.width(), grid.height(), i32::MAX);

                    let mut open = VecDeque::new();
                    open.push_back((tx, ty, 0));

                    while let Some((x, y, d)) = open.pop_front() {
                        if d < *dists.get(x, y) {
                            dists.set(x, y, d);
                        } else {
                            continue;
                        }

                        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = x + dx;
                            let ny = y + dy;

                            if *grid.get(nx, ny) == '.' {
                                open.push_back((nx, ny, d + 1));
                            }
                        }
                    }

                    // if i == 0 {
                    //     println!("{:4}", dists);
                    // }

                    let mut moves = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().filter_map(|(dx, dy)| {
                        let nx = unit.x + dx;
                        let ny = unit.y + dy;
                        let d = dists.get(nx, ny);
                        if *d == i32::MAX {
                            None
                        } else {
                            Some((d, ny, nx))
                        }
                    }).collect_vec();
                    moves.sort();
                    if VERBOSE { println!("{} moves: {:?}", i, moves); }

                    if let Some(mv) = moves.first() {
                        let (_, ny, nx) = *mv;

                        grid.set(unit.x, unit.y, '.');
                        unit.x = nx;
                        unit.y = ny;
                        grid.set(unit.x, unit.y, unit.ty);
                    }

                } else {
                    if VERBOSE { println!("{} no targets", i); }
                    continue; // end turn
                }
            }

            let mut targets = units.iter().cloned().enumerate().filter(|(j, t)|
                t.ty == enemy(unit.ty) &&
                t.x.abs_diff(unit.x) + t.y.abs_diff(unit.y) == 1 &&
                t.hp > 0
            ).collect_vec();
            targets.sort_by_key(|(j, t)| (t.hp, t.y, t.x));
            if VERBOSE { println!("{} atk targets: {:?}", i, targets); }
            if let Some(target) = targets.first() {
                units[target.0].hp -= 3;
                if units[target.0].hp <= 0 {
                    grid.set(target.1.x, target.1.y, '.');
                }
            }

            if VERBOSE { println!(""); }

            units[i] = unit;
        }

        println!("{}\n{:?}\n", grid, units);
    }
}


fn run2(title: &str, input: &str) {
    'OUTER: for elf_atk in 4.. {
        let mut grid = Grid::from(input);

        // Iterate over all units y,x
        // Identify targets (enemy units)
        // Identify open squares adjacent to target
        // If adj to target, do not move
        // Else if no open squares, end turn
        // Else find closest open square, tie-break on y,x
        // Move along shortest path; tie-break on y,x
        //
        // Attack:
        //  Find all adj enemies
        //  Select one with fewest hp, tie-break y,x
        //  hp -= attackpower
        //  (200hp, 3ap)

        let mut units = Vec::new();
        grid.for_each(|x, y, c| {
            if *c == 'E' || *c == 'G' {
                units.push(Unit { x, y, ty: *c, hp: 200 });
            }
        });

        const VERBOSE: bool = false;

        for turn in 0.. {

            units.sort_by_key(|u| (u.y, u.x));

            for i in 0..units.len() {
                let mut unit = units[i].clone();

                if unit.hp <= 0 {
                    continue;
                }

                if !units.iter().any(|t| t.hp > 0 && t.ty == enemy(unit.ty)) {
                    println!("combat complete: {}, atk={}, part2={}", turn, elf_atk, turn * (
                        units.iter().map(|u| u.hp.max(0)).sum::<i32>()
                    ));
                    return;
                }

                let in_range = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().any(|(dx, dy)| *grid.get(unit.x + dx, unit.y + dy) == enemy(unit.ty));
                if VERBOSE { println!("{} in range: {}", i, in_range); }

                if !in_range {
                    let mut targets = Vec::new();

                    {
                        let mut open = VecDeque::new();
                        open.push_back((unit.x, unit.y, 0));
                        let mut seen = HashSet::new();

                        while let Some((x, y, d)) = open.pop_front() {
                            if !seen.insert((x, y)) {
                                continue;
                            }

                            let mut is_target = false;
                            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                                let nx = x + dx;
                                let ny = y + dy;

                                if *grid.get(nx, ny) == '.' {
                                    open.push_back((nx, ny, d + 1));
                                } else if *grid.get(nx, ny) == enemy(unit.ty) {
                                    is_target = true;
                                }
                            }
                            if is_target {
                                targets.push((d, y, x));
                            }
                        }
                    }

                    targets.sort();
                    if let Some(target) = targets.first() {
                        let (_, ty, tx) = *target;
                        if VERBOSE { println!("{} target: {:?}", i, target); }

                        let mut dists = Grid::new_empty(grid.width(), grid.height(), i32::MAX);

                        let mut open = VecDeque::new();
                        open.push_back((tx, ty, 0));

                        while let Some((x, y, d)) = open.pop_front() {
                            if d < *dists.get(x, y) {
                                dists.set(x, y, d);
                            } else {
                                continue;
                            }

                            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                                let nx = x + dx;
                                let ny = y + dy;

                                if *grid.get(nx, ny) == '.' {
                                    open.push_back((nx, ny, d + 1));
                                }
                            }
                        }

                        // if i == 0 {
                        //     println!("{:4}", dists);
                        // }

                        let mut moves = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().filter_map(|(dx, dy)| {
                            let nx = unit.x + dx;
                            let ny = unit.y + dy;
                            let d = dists.get(nx, ny);
                            if *d == i32::MAX {
                                None
                            } else {
                                Some((d, ny, nx))
                            }
                        }).collect_vec();
                        moves.sort();
                        if VERBOSE { println!("{} moves: {:?}", i, moves); }

                        if let Some(mv) = moves.first() {
                            let (_, ny, nx) = *mv;

                            grid.set(unit.x, unit.y, '.');
                            unit.x = nx;
                            unit.y = ny;
                            grid.set(unit.x, unit.y, unit.ty);
                        }

                    } else {
                        if VERBOSE { println!("{} no targets", i); }
                        continue; // end turn
                    }
                }

                let mut targets = units.iter().cloned().enumerate().filter(|(j, t)|
                    t.ty == enemy(unit.ty) &&
                    t.x.abs_diff(unit.x) + t.y.abs_diff(unit.y) == 1 &&
                    t.hp > 0
                ).collect_vec();
                targets.sort_by_key(|(j, t)| (t.hp, t.y, t.x));
                if VERBOSE { println!("{} atk targets: {:?}", i, targets); }
                if let Some(target) = targets.first() {

                    let atk = if unit.ty == 'E' { elf_atk } else { 3 };
                    units[target.0].hp -= atk;

                    if units[target.0].hp <= 0 {
                        if units[target.0].ty == 'E' {
                            println!("elf died");
                            continue 'OUTER;
                        }
                        grid.set(target.1.x, target.1.y, '.');
                    }
                }

                if VERBOSE { println!(""); }

                units[i] = unit;
            }

            println!("{}\n{:?}\n", grid, units);
        }
    }
}

const INPUT_DEMO: &str = "#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
";

const INPUT_DEMO2: &str = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("demo 2", INPUT_DEMO2);
    // run("input", &std::fs::read_to_string("15/input.txt").unwrap());

    run2("demo 2", INPUT_DEMO2);
    run2("input", &std::fs::read_to_string("15/input.txt").unwrap());
}
