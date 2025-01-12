// Part 1: 25 mins
// Part 1+2: 27 mins

use std::collections::{HashMap, HashSet};

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let grid = Grid::from(input);

    let mut elves = HashSet::new();
    grid.for_each(|x, y, c| {
        if *c == '#' {
            elves.insert((x, y));
        }
    });

    let check = [
        [(-1,-1),(0,-1),(1,-1)], // NW N NE
        [(-1,1),(0,1),(1,1)], // SW S SE
        [(-1,-1),(-1,0),(-1,1)], // NW W SW
        [(1,-1),(1,0),(1,1)], // NE E SE
    ];

    for i in 0.. {
        let mut proposed = HashMap::new();

        for elf in &elves {
            let all_ok = check.iter().all(|c| c.iter().all(|(dx, dy)| !elves.contains(&(elf.0 + dx, elf.1 + dy))));
            if !all_ok {
                for k in 0..4 {
                    let c = check[(i + k) % 4];
                    let ok = c.iter().all(|(dx, dy)| !elves.contains(&(elf.0 + dx, elf.1 + dy)));
                    if ok {
                        *proposed.entry((elf.0 + c[1].0, elf.1 + c[1].1)).or_insert(0) += 1;
                        // println!("{:?} proposes {:?}", elf, c);
                        break;
                    }
                }
            }
        }

        let mut new_elves = HashSet::new();
        'ELF: for elf in &elves {
            let all_ok = check.iter().all(|c| c.iter().all(|(dx, dy)| !elves.contains(&(elf.0 + dx, elf.1 + dy))));
            if !all_ok {
                for k in 0..4 {
                    let c = check[(i + k) % 4];
                    let ok = c.iter().all(|(dx, dy)| !elves.contains(&(elf.0 + dx, elf.1 + dy)));
                    if ok {
                        if proposed[&(elf.0 + c[1].0, elf.1 + c[1].1)] == 1 {
                            new_elves.insert((elf.0 + c[1].0, elf.1 + c[1].1));
                            // println!("{:?} moves {:?}", elf, c);
                            continue 'ELF;
                        } else {
                            // println!("{:?} blocked {:?}", elf, c);
                            break;
                        }
                    }
                }
            }

            new_elves.insert(*elf);
        }

        if elves == new_elves {
            println!("{} part 2: {}", title, i + 1);
            break;
        }

        elves = new_elves;

        let x0 = elves.iter().map(|(x, y)| x).min().unwrap();
        let x1 = elves.iter().map(|(x, y)| x).max().unwrap();
        let y0 = elves.iter().map(|(x, y)| y).min().unwrap();
        let y1 = elves.iter().map(|(x, y)| y).max().unwrap();

        // println!("After round {}", i+1);
        // for y in y0..=y1 {
        //     for x in x0..=x1 {
        //         print!("{}", if elves.contains(&(x, y)) { '#' } else { '.' });
        //     }
        //     println!();
        // }
        // println!();

        if i == 9 {
            println!("{} part 1: {}", title, (x1 - x0 + 1) * (y1 - y0 + 1) - elves.len() as i32);
        }
    }
}

const INPUT_DEMO: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

const INPUT_DEMO2: &str = ".....
..##.
..#..
.....
..##.
.....
";

fn main() {
    run("demo", INPUT_DEMO);
    // run("demo", INPUT_DEMO2);
    run("input", &std::fs::read_to_string("23/input.txt").unwrap());
}
