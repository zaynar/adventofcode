// Part 1: 15 mins
// Part 1+2: 36 mins

use std::collections::HashSet;
use aocgrid::Grid;

use itertools::Itertools;

fn push(pos: (i32, i32), dir: (i32, i32), grid: &Grid<char>) -> ((i32, i32), Grid<char>) {
    let nx = pos.0 + dir.0;
    let ny = pos.1 + dir.1;
    match grid.get(nx, ny) {
        '.' => ((nx, ny), grid.clone()),
        '#' => (pos, grid.clone()),
        'O' => {
            for rocks in 1i32.. {
                let ex = pos.0 + dir.0 * rocks;
                let ey = pos.1 + dir.1 * rocks;
                if *grid.get(ex, ey) == '#' {
                    return (pos, grid.clone());
                } else if *grid.get(ex, ey) == '.' {
                    let mut ret = grid.clone();
                    ret.set(ex, ey, 'O');
                    ret.set(nx, ny, '.');
                    return ((nx, ny), ret);
                } else {
                    assert_eq!(*grid.get(ex, ey), 'O');
                }
            }
            panic!()
        },
        _ => panic!()
    }
}

fn run(title: &str, input: &str) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from(grid);
    let moves = moves.replace("\n", "").chars().collect_vec();

    let mut pos = (0, 0);

    // println!("{}", grid);

    grid.for_each(|x, y, c| {
        if *c == '@' {
            pos = (x as i32, y as i32);
            *c = '.';
        }
    });

    // println!("{:?}", grid);
    // println!("{:?}", moves);
    // println!("{:?}", pos);

    for m in &moves {
        let dir = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };
        (pos, grid) = push(pos, dir, &grid);
    }

    let mut gps = 0;
    grid.for_each(|x, y, c| {
        if *c == 'O' {
            gps += 100*y + x;
        }
    });

    // println!("{}", grid);

    println!("{} part 1: {}", title, gps);
}

// Find all connected boxes
fn boxes(found: &mut HashSet<(i32, i32)>, pos: (i32, i32), dir: (i32, i32), grid: &Grid<char>) {
    found.insert(pos);

    let nx = pos.0 + dir.0;
    let ny = pos.1 + dir.1;
    match grid.get(nx, ny) {
        '[' => {
            found.insert((nx, ny));
            boxes(found, (nx, ny), dir, grid);
            if dir.1 != 0 {
                boxes(found, (nx + 1, ny), dir, grid);
            }
        }
        ']' => {
            found.insert((nx, ny));
            boxes(found, (nx, ny), dir, grid);
            if dir.1 != 0 {
                boxes(found, (nx - 1, ny), dir, grid);
            }
        }
        _ => {}
    }
}

fn push2(pos: (i32, i32), dir: (i32, i32), grid: &Grid<char>) -> ((i32, i32), Grid<char>) {
    let nx = pos.0 + dir.0;
    let ny = pos.1 + dir.1;

    match grid.get(nx, ny) {
        '.' => ((nx, ny), grid.clone()),
        '#' => (pos, grid.clone()),
        '[' | ']' => {

            let mut rocks = HashSet::new();
            boxes(&mut rocks, (nx, ny), dir, grid);
            if *grid.get(nx, ny) == '[' && dir.1 != 0 {
                boxes(&mut rocks, (nx + 1, ny), dir, grid);
            } else if *grid.get(nx, ny) == ']' && dir.1 != 0 {
                boxes(&mut rocks, (nx - 1, ny), dir, grid);
            }
            // println!("pushing {} {:?}", rocks.len(), rocks);

            for (rx, ry) in &rocks {
                let rnx = rx + dir.0;
                let rny = ry + dir.1;
                if *grid.get(rnx, rny) == '#' {
                    return (pos, grid.clone());
                }
            }

            let mut new = grid.clone();
            for (rx, ry) in &rocks {
                new.set(*rx, *ry, '.');
            }
            for (rx, ry) in &rocks {
                let rnx = rx + dir.0;
                let rny = ry + dir.1;
                new.set(rnx, rny, *grid.get(*rx, *ry));
            }

            return ((nx, ny), new);
        },
        _ => panic!()
    }
}

fn run2(title: &str, input: &str) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from(grid.lines().map(|line| line.chars().map(|c|
        match c {
            '#' => vec!['#','#'],
            'O' => vec!['[',']'],
            '.' => vec!['.','.'],
            '@' => vec!['@','.'],
            _ => panic!()
        }
    ).concat()).collect_vec());
    let moves = moves.replace("\n", "").chars().collect_vec();

    let mut pos = (0, 0);

    grid.for_each(|x, y, c| {
        if *c == '@' {
            pos = (x as i32, y as i32);
            *c = '.';
        }
    });

    println!("{}", grid);

    // println!("{:?}", grid);
    // println!("{:?}", moves);
    // println!("{:?}", pos);

    for (i, m) in moves.iter().enumerate() {
        // println!("====== {}, {:?} {}", i, pos, m);

        let dir = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };
        (pos, grid) = push2(pos, dir, &grid);
    }

    println!("{}", grid.map_coords(|x, y, c| {
        if (x, y) == pos {
            assert_eq!(c, '.');
            '@'
        } else {
            c
        }
    }));

    let mut gps = 0;
    grid.for_each(|x, y, c| {
        if *c == '[' {
            gps += 100*y + x;
        }
    });


    println!("{} part 2: {}", title, gps);
}

const INPUT_DEMO: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

fn main() {
    // run("demo", INPUT_DEMO);
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("15/input.txt").unwrap());
}
