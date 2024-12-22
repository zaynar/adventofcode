// Part 1: 6 mins
// Part 1+2: 10 mins

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let mut grid = Grid::from(input);

    for i in 0.. {
        let mut new = grid.clone();
        new.for_each(|x, y, c| {
            let ns = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
            if *c == 'L' && ns.iter().filter(|(dx, dy)| {
                grid.try_get(x + dx, y + dy) == Some(&'#')
            }).count() == 0 {
                *c = '#';
            } else if *c == '#' && ns.iter().filter(|(dx, dy)| {
                grid.try_get(x + dx, y + dy) == Some(&'#')
            }).count() >= 4 {
                *c = 'L';
            }
        });
        if grid == new {
            let mut part1 = 0;
            new.for_each(|x, y, c| if *c == '#' { part1 += 1; });
            println!("{} part 1: {}", title, part1);
            break;
        }

        grid = new;
    }
}

fn run2(title: &str, input: &str) {
    let mut grid = Grid::from(input);

    for i in 0.. {
        let mut new = grid.clone();
        new.for_each(|x, y, c| {
            let ns = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

            let occ = ns.iter().filter(|(dx, dy)| {
                for d in 1.. {
                    let c = grid.try_get(x + dx*d, y + dy*d);
                    if c.is_none() {
                        return false;
                    }
                    if c == Some(&'#') {
                        return true;
                    }
                    if c == Some(&'L') {
                        return false;
                    }
                }
                return false;
            }).count();

            if *c == 'L' && occ == 0 {
                *c = '#';
            } else if *c == '#' && occ >= 5 {
                *c = 'L';
            }
        });

        // println!("{}\n", new);

        if grid == new {
            let mut part1 = 0;
            new.for_each(|x, y, c| if *c == '#' { part1 += 1; });
            println!("{} part 1: {}", title, part1);
            break;
        }

        grid = new;
    }

    // println!("{}", grid);


    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("11/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("11/input.txt").unwrap());
}
