// Part 1: 10 mins
// Part 1+2: 12 mins

use std::collections::HashSet;

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let mut grid = Grid::from(input).map(|c| c.to_digit(10).unwrap());

    let mut part1 = 0;
    // for step in 0..100 {
    for step in 0.. {

        let mut flashers = Vec::new();
        grid.for_each_mut(|x, y, c| {
            *c += 1;
            if *c == 10 {
                flashers.push((x, y));
            }
        });

        // println!("{:2}", grid);

        while let Some((x, y)) = flashers.pop() {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx, dy) == (0, 0) { continue; }

                    if let Some(n) = grid.try_get_mut(x + dx, y + dy) {
                        *n += 1;
                        if *n == 10 {
                            flashers.push((x + dx, y + dy));
                        }
                    }
                }
            }
        }

        // println!("{:2}", grid);

        let mut all = true;
        grid.for_each_mut(|x, y, c| {
            if *c > 9 {
                part1 += 1;
                *c = 0;
            } else {
                all = false;
            }
        });

        if all {
            println!("{} part 2: {}", title, step + 1);
            break;
        }

        // println!("{:2}", grid);

        if step == 99 {
            println!("{} part 1: {}", title, part1);
        }

    }
}


const INPUT_DEMO0: &str = "11111
11111
11911
11111
11111
";

const INPUT_DEMO: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("11/input.txt").unwrap());
}
