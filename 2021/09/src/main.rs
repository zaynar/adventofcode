// Part 1: 6 mins
// Part 1+2: 11 mins

use std::collections::HashSet;

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let grid = Grid::from(input).map(|c| c.to_digit(10).unwrap());

    let mut part1 = 0;
    let mut basins = vec![];

    grid.for_each(|x, y, c| {
        let mut low = true;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(n) = grid.try_get(x + dx, y + dy) {
                if n <= c {
                    low = false;
                }
            }
        }
        if low {
            // println!("low {} {} {}", x, y, c);
            part1 += 1 + *c;

            let mut open = vec![(x, y)];
            let mut seen = HashSet::new();

            while let Some((x, y)) = open.pop() {
                if !seen.insert((x, y)) {
                    continue;
                }

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if let Some(n) = grid.try_get(x + dx, y + dy) {
                        if *n != 9 {
                            open.push((x + dx, y + dy));
                        }
                    }
                }
            }

            // println!("basin {}", seen.len());
            basins.push(seen.len());
        }
    });

    println!("{} part 1: {}", title, part1);

    basins.sort();
    println!("{} part 2: {}", title, basins[(basins.len()-3)..].iter().product::<usize>());
}

const INPUT_DEMO: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
