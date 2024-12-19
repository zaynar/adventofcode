// Part 1: 7 mins
// Part 2: 10 mins

use std::collections::HashMap;

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let mut grid = Grid::from(input);

    let adj = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut seen = HashMap::new();

    for i in 0.. {

        if let Some(n) = seen.get(&grid) {
            println!("loop {} {} = {}", n, i, i-n);

            if (1000000000 - i) % (i - n) == 0 {
                let mut wood = 0;
                let mut lum = 0;
                grid.for_each(|x, y, c| {
                    if *c == '|' {
                        wood += 1;
                    } else if *c == '#' {
                        lum += 1;
                    }
                });

                println!("{} part 2: {}", title, wood * lum);
                break;
            }

        } else {
            seen.insert(grid.clone(), i);
        }

        let mut grid2 = grid.clone();
        grid2.for_each(|x, y, c| {
            match c {
                '.' => {
                    if adj.iter().filter(|a| grid.try_get(x + a.0, y + a.1) == Some(&'|')).count() >= 3 {
                        *c = '|';
                    }
                }
                '|' => {
                    if adj.iter().filter(|a| grid.try_get(x + a.0, y + a.1) == Some(&'#')).count() >= 3 {
                        *c = '#';
                    }
                }
                '#' => {
                    if adj.iter().filter(|a| grid.try_get(x + a.0, y + a.1) == Some(&'#')).count() >= 1
                    && adj.iter().filter(|a| grid.try_get(x + a.0, y + a.1) == Some(&'|')).count() >= 1 {
                    } else {
                        *c = '.';
                    }
                }
                _ => panic!()
            }
        });
        grid = grid2;

        // println!("{}", grid);

        if i == 10 {
            let mut wood = 0;
            let mut lum = 0;
            grid.for_each(|x, y, c| {
                if *c == '|' {
                    wood += 1;
                } else if *c == '#' {
                    lum += 1;
                }
            });

            println!("{} part 1: {}", title, wood * lum);
        }
    }
}

const INPUT_DEMO: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap());
}
