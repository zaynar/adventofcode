// Part 1: 15 mins
// Part 1+2: 36 mins

use std::collections::HashSet;

use itertools::Itertools;

fn push(pos: (i32, i32), dir: (i32, i32), grid: &Vec<Vec<char>>) -> ((i32, i32), Vec<Vec<char>>) {
    let nx = pos.0 + dir.0;
    let ny = pos.1 + dir.1;
    match grid[ny as usize][nx as usize] {
        '.' => ((nx, ny), grid.clone()),
        '#' => (pos, grid.clone()),
        'O' => {
            for rocks in 1i32.. {
                let ex = pos.0 + dir.0 * rocks;
                let ey = pos.1 + dir.1 * rocks;
                if grid[ey as usize][ex as usize] == '#' {
                    return (pos, grid.clone());
                } else if grid[ey as usize][ex as usize] == '.' {
                    let mut ret = grid.clone();
                    ret[ey as usize][ex as usize] = 'O';
                    ret[ny as usize][nx as usize] = '.';
                    return ((nx, ny), ret);
                } else {
                    assert_eq!(grid[ey as usize][ex as usize], 'O');
                }
            }
            panic!()
        },
        _ => panic!()
    }
}

fn run(title: &str, input: &str) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid = grid.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let moves = moves.replace("\n", "").chars().collect_vec();

    let mut pos = (0, 0);

    let h = grid.len();
    let w = grid[0].len();
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '@' {
                pos = (x as i32, y as i32);
                grid[y][x] = '.';
            }
        }
    }

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
    for y in 0..h {
        for x in 0..w {
            // print!("{}", grid[y][x]);
            if grid[y][x] == 'O' {
                gps += 100*y + x;
            }
        }
        // println!();
    }

    println!("{} part 1: {}", title, gps);
}

// Find all connected boxes
fn boxes(found: &mut HashSet<(i32, i32)>, pos: (i32, i32), dir: (i32, i32), grid: &Vec<Vec<char>>) {
    found.insert(pos);

    let nx = pos.0 + dir.0;
    let ny = pos.1 + dir.1;
    match grid[ny as usize][nx as usize] {
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

fn push2(pos: (i32, i32), dir: (i32, i32), grid: &Vec<Vec<char>>) -> ((i32, i32), Vec<Vec<char>>) {
    let nx = pos.0 + dir.0;
    let ny = pos.1 + dir.1;

    match grid[ny as usize][nx as usize] {
        '.' => ((nx, ny), grid.clone()),
        '#' => (pos, grid.clone()),
        '[' | ']' => {

            let mut rocks = HashSet::new();
            boxes(&mut rocks, (nx, ny), dir, grid);
            if grid[ny as usize][nx as usize] == '[' && dir.1 != 0 {
                boxes(&mut rocks, (nx + 1, ny), dir, grid);
            } else if grid[ny as usize][nx as usize] == ']' && dir.1 != 0 {
                boxes(&mut rocks, (nx - 1, ny), dir, grid);
            }
            println!("pushing {} {:?}", rocks.len(), rocks);

            for (rx, ry) in &rocks {
                let rnx = rx + dir.0;
                let rny = ry + dir.1;
                if grid[rny as usize][rnx as usize] == '#' {
                    return (pos, grid.clone());
                }
            }

            let mut new = grid.clone();
            for (rx, ry) in &rocks {
                new[*ry as usize][*rx as usize] = '.';
            }
            for (rx, ry) in &rocks {
                let rnx = rx + dir.0;
                let rny = ry + dir.1;
                new[rny as usize][rnx as usize] = grid[*ry as usize][*rx as usize];
            }

            return ((nx, ny), new);
        },
        _ => panic!()
    }
}

fn run2(title: &str, input: &str) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid = grid.lines().map(|line| line.chars().map(|c|
        match c {
            '#' => vec!['#','#'],
            'O' => vec!['[',']'],
            '.' => vec!['.','.'],
            '@' => vec!['@','.'],
            _ => panic!()
        }
    ).concat()).collect_vec();
    let moves = moves.replace("\n", "").chars().collect_vec();

    let mut pos = (0, 0);

    let h = grid.len();
    let w = grid[0].len();
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '@' {
                pos = (x as i32, y as i32);
                grid[y][x] = '.';
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            print!("{}", grid[y][x]);
        }
        println!();
    }

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

        // for y in 0..h {
        //     for x in 0..w {
        //         if (x as i32, y as i32) == pos {
        //             if grid[y][x] != '.' { println!("==== BAD ====");}
        //             print!("@");
        //         } else {
        //             print!("{}", grid[y][x]);
        //         }
        //     }
        //     println!();
        // }

    }

    let mut gps = 0;
    for y in 0..h {
        for x in 0..w {
            if (x as i32, y as i32) == pos {
                assert_eq!(grid[y][x], '.');
                print!("@");
            } else {
                print!("{}", grid[y][x]);
            }
            if grid[y][x] == '[' {
                gps += 100*y + x;
            }
        }
        println!();
    }
    println!();

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
