// Part 1: 16 mins
// Part 1+2: 27 mins

use std::collections::HashSet;

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut grid = Grid::from(input);

    let mut carts = Vec::new();
    grid.for_each(|x, y, c| {
        match c {
            '<' => {
                *c = '-';
                carts.push((y, x, (-1, 0), 0, false));
            }
            '>' => {
                *c = '-';
                carts.push((y, x, (1, 0), 0, false));
            }
            '^' => {
                *c = '|';
                carts.push((y, x, (0, -1), 0, false));
            }
            'v' => {
                *c = '|';
                carts.push((y, x, (0, 1), 0, false));
            }
            _ => ()
        }
    });

    // println!("{}", grid);
    // println!("{:?}", carts);

    'OUTER: for i in 0.. {
        carts.sort();

        // println!("{:?}", carts);
        let mut grid2 = grid.clone();
        for c in &carts {
            let (y, x, d, n, crashed) = *c;
            if crashed {
                // grid2.set(x, y, 'X');
            } else {
                grid2.set(x, y, match d {
                    (-1, 0) => '<',
                    (1, 0) => '>',
                    (0, -1) => '^',
                    (0, 1) => 'v',
                    _ => panic!(),
                });
            }
        }
        // println!("{}\n", grid2);

        let mut busy = HashSet::new();
        for c in &carts {
            let (y, x, d, n, crashed) = *c;
            if !crashed {
                busy.insert((x, y));
            }
        }

        for j in 0..carts.len() {
            let (y, x, d, n, mut crashed) = carts[j];
            if crashed {
                continue;
            }

            let nx = x + d.0;
            let ny = y + d.1;
            busy.remove(&(x, y));
            if !busy.insert((nx, ny)) {
                println!("{} part 1: {},{}", title, nx, ny);

                carts[j].4 = true;
                for c in &mut carts {
                    if (c.1, c.0) == (nx, ny) {
                        c.4 = true;
                    }
                }

                continue;
            }

            let mut nd = d;
            let mut nn = n;
            match grid.get(nx, ny) {
                '/' => {
                    nd = (-d.1, -d.0);
                }
                '\\' => {
                    nd = (d.1, d.0);
                }
                '+' => {
                    nd = match nn % 3 {
                        0 => (d.1, -d.0),
                        1 => (d.0, d.1),
                        2 => (-d.1, d.0),
                        _ => panic!(),
                    };
                    nn += 1;
                }
                _ => ()
            }

            carts[j] = (ny, nx, nd, nn, crashed);
        }

        let alive = carts.iter().filter(|(_, _, _, _, crashed)| !crashed).collect_vec();
        if alive.len() == 1 {
            let c = alive[0];
            println!("{} part 2: {},{}", title, c.1, c.0);
            break 'OUTER;
        }
    }
}

const INPUT_DEMO: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/ "#;

  const INPUT_DEMO2: &str = r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

fn main() {
    run("demo", INPUT_DEMO2);
    run("input", &std::fs::read_to_string("13/input.txt").unwrap());
}
