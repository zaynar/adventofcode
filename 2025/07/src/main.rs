// Part 1: 6 mins
// Part 1+2: 10 mins

use std::collections::HashMap;

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let grid = Grid::from(input);

    let start = grid.find(&'S').unwrap();

    let mut part1 = 0;

    // let mut beams = vec![start.0];
    // for y in 1..grid.height() {
    //     let mut newbeams = vec![];
    //     for &x in &beams {
    //         if *grid.get(x, y) == '.' {
    //             newbeams.push(x);
    //         } else if *grid.get(x, y) == '^' {
    //             newbeams.push(x-1);
    //             newbeams.push(x+1);
    //             part1 += 1;
    //         } else {
    //             panic!();
    //         }
    //     }
    //     newbeams.sort();
    //     newbeams.dedup();
    //     beams = newbeams;
    // }

    let mut beams = HashMap::new();
    beams.insert(start.0, 1);

    for y in 1..grid.height() {
        let mut newbeams = HashMap::new();
        for (&x, &n) in &beams {
            if *grid.get(x, y) == '.' {
                *newbeams.entry(x).or_default() += n;
            } else if *grid.get(x, y) == '^' {
                *newbeams.entry(x-1).or_default() += n;
                *newbeams.entry(x+1).or_default() += n;
                part1 += 1;
            } else {
                panic!();
            }
        }
        beams = newbeams;
    }

    let part2 = beams.values().sum::<u64>();

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
