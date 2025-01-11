// Part 1: 14 mins
// Part 1+2: 50 mins

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str, size: i32) {
    let (grid, path) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from(grid);
    let path = path.split_inclusive(|c| c == 'L' || c == 'R').collect_vec();

    let start = grid.find(&'.').unwrap();

    let mut dir = (1, 0);
    let mut pos = start;

    for step in &path {
        let n: u32 = step[0..step.len() - 1].parse().unwrap();
        let turn = step.chars().last().unwrap();

        for i in 0..n {
            let mut np = (pos.0 + dir.0, pos.1 + dir.1);
            let mut next = grid.try_get(np.0, np.1);
            if next.is_none() || next == Some(&' ') {
                np.0 -= dir.0 * 200;
                np.1 -= dir.1 * 200;
            }
            next = grid.try_get(np.0, np.1);
            while next.is_none() || next == Some(&' ') {
                np.0 += dir.0;
                np.1 += dir.1;
                next = grid.try_get(np.0, np.1);
            }

            if next == Some(&'#') {
                break;
            }

            pos = np;

            grid.set(pos.0, pos.1, '*');
        }

        match turn {
            'R' => dir = (-dir.1, dir.0),
            'L' => dir = (dir.1, -dir.0),
            '\n' => (),
            _ => panic!()
        };
    }

    // println!("{}", grid);
    // println!("{:?}", (pos, dir));

    let score = (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + match dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!()
    };

    println!("{} part 1: {}", title, score);


    let mut dir = (1, 0);
    let mut pos = start;

    for step in &path {
        let n: u32 = step[0..step.len() - 1].parse().unwrap();
        let turn = step.chars().last().unwrap();

        for i in 0..n {
            let mut np = (pos.0 + dir.0, pos.1 + dir.1);
            let mut nd = dir;
            if region(np, size) == 0 {
                let rx = pos.0 % size;
                let ry = pos.1 % size;
                match (region(pos, size), dir) {
                    // (1, (-1, 0)) => panic!(),
                    // (1, (1, 0)) => panic!(),
                    // (1, (0, -1)) => panic!(),
                    // (2, (-1, 0)) => panic!(),
                    // (2, (0, -1)) => panic!(),
                    // (2, (0, 1)) => panic!(),
                    // (3, (0, -1)) => panic!(),
                    // (3, (0, 1)) => panic!(),
                    // (4, (1, 0)) => panic!(),
                    // (5, (-1, 0)) => panic!(),
                    // (5, (0, 1)) => panic!(),
                    // (6, (0, -1)) => panic!(),
                    // (6, (1, 0)) => panic!(),
                    // (6, (0, 1)) => panic!(),

                    (1, (-1, 0)) => {
                        np = (0, size*3 - ry - 1);
                        nd = (1, 0);
                    },
                    (1, (0, -1)) => { // 1 -> 6
                        np = (0, size*3 + rx);
                        nd = (1, 0);
                    },
                    (2, (0, -1)) => { // 2 -> 6
                        np = (rx, size*4- 1);
                        nd = (0, -1);
                    },
                    (2, (0, 1)) => { // 2 -> 3
                        np = (size*2-1, size + rx);
                        nd = (-1, 0);
                    },
                    (2, (1, 0)) => { // 2 -> 5
                        np = (size*2-1, size*3-1 - ry);
                        nd = (-1, 0);
                    },
                    (3, (-1, 0)) => { // 3 -> 4
                        np = (ry, size*2);
                        nd = (0, 1);
                    },
                    (3, (1, 0)) => { // 3 -> 2
                        np = (2*size + ry, size-1);
                        nd = (0, -1);
                    },
                    (4, (0, -1)) => { // 4 -> 3
                        np = (size, size + rx);
                        nd = (1, 0);
                    },
                    (4, (-1, 0)) => {
                        np = (size, size - ry - 1);
                        nd = (1, 0);
                    },
                    (5, (0, 1)) => {
                        np = (size - 1, size*3 + rx);
                        nd = (-1, 0);
                    },
                    (5, (1, 0)) => { // 5 -> 2
                        np = (size*3 - 1, size-1 - ry);
                        nd = (-1, 0);
                    },
                    (6, (-1, 0)) => { // 6 -> 1
                        np = (size + ry, 0);
                        nd = (0, 1);
                    },
                    (6, (1, 0)) => { // 6 -> 5
                        np = (size + ry, size*3-1);
                        nd = (0, -1);
                    },
                    (6, (0, 1)) => { // 6 -> 2
                        np = (size + rx, 0);
                        nd = (0, 1);
                    },
                    _ => panic!("not handled {:?} {:?} @ {:?}", region(pos, size), dir, pos),
                }
            }

            println!("{:?}/{:?} {:?}/{:?} -- {:?}", pos, dir, np, nd, (region(pos, size), dir));

            assert_ne!(region(np, size), 0);
            if grid.get(np.0, np.1) == &'#' {
                break;
            }

            pos = np;
            dir = nd;

            grid.set(pos.0, pos.1, '*');
        }

        match turn {
            'R' => dir = (-dir.1, dir.0),
            'L' => dir = (dir.1, -dir.0),
            '\n' => (),
            _ => panic!()
        };
    }

    // println!("{}", grid);
    // println!("{:?}", (pos, dir));

    let score = (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + match dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!()
    };

    println!("{} part 2: {}", title, score);
}

// fn region((x, y): (i32, i32), size: i32) -> i32 {
//     match (x / size, y / size) {
//         (2, 0) => 1,
//         (0, 1) => 2,
//         (1, 1) => 3,
//         (2, 1) => 4,
//         (2, 2) => 5,
//         (3, 2) => 6,
//         _ => 0,
//     }
// }
fn region((x, y): (i32, i32), size: i32) -> i32 {
    if x < 0 || y < 0 {
        return 0;
    }
    match (x / size, y / size) {
        (1, 0) => 1,
        (2, 0) => 2,
        (1, 1) => 3,
        (0, 2) => 4,
        (1, 2) => 5,
        (0, 3) => 6,
        _ => 0,
    }
}

const INPUT_DEMO: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

fn main() {
    // run("demo", INPUT_DEMO, 4);
    run("input", &std::fs::read_to_string("22/input.txt").unwrap(), 50);
}
