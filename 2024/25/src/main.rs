use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let grids: Vec<Grid<char>> = input.split("\n\n").map(|g| Grid::from(g)).collect();
    let grids = grids.iter().map(|g| {
        let mut v = vec![];
        for x in 0..g.width() {
            let mut h = 0;
            for y in 0..g.height() {
                if *g.get(x, y) == '#' {
                    h += 1;
                }
            }
            v.push(h);
        }
        (*g.get(0, 0) == '#', v)
    }).collect_vec();

    // println!("{:?}", grids);

    let mut part1 = 0;
    for lock in &grids {
        if !lock.0 { continue; }

        for key in &grids {
            if key.0 { continue; }

            // println!("{:?} {:?}", lock.1, key.1);
            if lock.1.iter().zip(key.1.iter()).all(|(a, b)| a + b <= 7) {
                part1 += 1;
            }
        }
    }

    println!("{} part 1: {}", title, part1);
}

const INPUT_DEMO: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("25/input.txt").unwrap());
}
