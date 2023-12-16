use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Beam {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    c: char,
    n: bool,
    s: bool,
    e: bool,
    w: bool,
}

fn process(mut grid: Vec<Vec<Cell>>, ox: usize, oy: usize, odx: isize, ody: isize) -> usize {
    loop {
        let mut changed = false;
        let mut new_grid = grid.clone();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let mut g = grid[y][x].clone();
                let e = (x > 0 && grid[y][x-1].e) || (x == ox && y == oy && odx == 1);
                let w = (x < grid[y].len() - 1 && grid[y][x+1].w) || (x == ox && y == oy && odx == -1);
                let s = (y > 0 && grid[y-1][x].s) || (x == ox && y == oy && ody == 1);
                let n = (y < grid.len() - 1 && grid[y+1][x].n) || (x == ox && y == oy && ody == -1);
                match grid[y][x].c {
                    '.' => {
                        if e {
                            g.e = true;
                        }
                        if w {
                            g.w = true;
                        }
                        if s {
                            g.s = true;
                        }
                        if n {
                            g.n = true;
                        }
                    },
                    '/' => {
                        if e {
                            g.n = true;
                        }
                        if w {
                            g.s = true;
                        }
                        if s {
                            g.w = true;
                        }
                        if n {
                            g.e = true;
                        }
                    },
                    '\\' => {
                        if e {
                            g.s = true;
                        }
                        if w {
                            g.n = true;
                        }
                        if s {
                            g.e = true;
                        }
                        if n {
                            g.w = true;
                        }
                    },
                    '|' => {
                        if e {
                            g.n = true;
                            g.s = true;
                        }
                        if w {
                            g.n = true;
                            g.s = true;
                        }
                        if s {
                            g.s = true;
                        }
                        if n {
                            g.n = true;
                        }
                    },
                    '-' => {
                        if e {
                            g.e = true;
                        }
                        if w {
                            g.w = true;
                        }
                        if s {
                            g.e = true;
                            g.w = true;
                        }
                        if n {
                            g.e = true;
                            g.w = true;
                        }
                    },
                    _ => {}
                }
                if grid[y][x] != g {
                    changed = true;
                }
                new_grid[y][x] = g;
            }
        }
        grid = new_grid;
        if !changed {
            break;
        }
    }

    // for row in grid.iter() {
    //     println!("{}", row.iter().map(|c| if c.c != '.' { c.c } else {
    //         match (c.n, c.e, c.s, c.w) {
    //             (true, false, false, false) => '^',
    //             (false, true, false, false) => '>',
    //             (false, false, true, false) => 'v',
    //             (false, false, false, true) => '<',
    //             (false, false, false, false) => '.',
    //             _ => '?',
    //         }
    //     }).collect::<String>());
    // }

    let count: usize = grid.iter().map(|row|
        row.iter().map(|c| match (c.n, c.e, c.s, c.w) {
                (false, false, false, false) => 0,
                _ => 1,
        }).sum::<usize>()

    ).sum();
    // println!("Answer 1: {}", count);}
    count
}

fn main() {
    let mut grid: Vec<Vec<Cell>> = fs::read_to_string("input").unwrap().lines().map(|line| line.chars().map(|c| Cell { c, n: false, s: false, e: false, w: false }).collect()).collect();

    println!("Answer 1: {}", process(grid.clone(), 0, 0, 1, 0));

    println!("Answer 2: {}", (0..grid.len()).map(|y| process(grid.clone(), 0, y, 1, 0)).max().unwrap());
    println!("Answer 2: {}", (0..grid.len()).map(|y| process(grid.clone(), grid[0].len()-1, y, -1, 0)).max().unwrap());
    println!("Answer 2: {}", (0..grid[0].len()).map(|x| process(grid.clone(), x, 0, 0, 1)).max().unwrap());
    println!("Answer 2: {}", (0..grid[0].len()).map(|x| process(grid.clone(), x, grid.len()-1, 0, -1)).max().unwrap());
}

fn main_alt() {
    let grid: Vec<Vec<char>> = fs::read_to_string("input").unwrap().lines().map(|line| line.chars().collect()).collect();
    let mut energised: Vec<Vec<bool>> = grid.iter().map(|row| row.iter().map(|_| false).collect()).collect();

    let mut beams = Vec::new();
    beams.push(Beam { x: -1, y: 0, dx: 1, dy: 0 });
    energised[0][0] = true;

    for i in 0..1000 {
        beams = beams.iter().flat_map(|beam| {

            if !(beam.x < 0 || beam.y < 0 || beam.x >= grid[0].len() as isize || beam.y >= grid.len() as isize) {
                energised[beam.y as usize][beam.x as usize] = true;
            }

            let nx = beam.x + beam.dx;
            let ny = beam.y + beam.dy;
            if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
                return vec!();
            }
            energised[ny as usize][nx as usize] = true;
            match grid[ny as usize][nx as usize] {
                '.' => {
                    return vec!(Beam { dx: beam.dx, dy: beam.dy, x: nx, y: ny });
                },
                '/' => {
                    if beam.dx != 0 {
                        return vec!(Beam { dx: 0, dy: -beam.dx, x: nx, y: ny });
                    } else {
                        return vec!(Beam { dx: -beam.dy, dy: 0, x: nx, y: ny });
                    }
                },
                '\\' => {
                    if beam.dx != 0 {
                        return vec!(Beam { dx: 0, dy: beam.dx, x: nx, y: ny });
                    } else {
                        return vec!(Beam { dx: beam.dy, dy: 0, x: nx, y: ny });
                    }
                },
                '|' => {
                    if beam.dx != 0 {
                        return vec!(
                            Beam { dx: 0, dy: -1, x: nx, y: ny },
                            Beam { dx: 0, dy: 1, x: nx, y: ny }
                        );
                    } else {
                        return vec!(Beam { dx: beam.dx, dy: beam.dy, x: nx, y: ny });
                    }
                },
                '-' => {
                    if beam.dy != 0 {
                        return vec!(
                            Beam { dx: -1, dy: 0, x: nx, y: ny },
                            Beam { dx: 1, dy: 0, x: nx, y: ny }
                        );
                    } else {
                        return vec!(Beam { dx: beam.dx, dy: beam.dy, x: nx, y: ny });
                    }
                },
            _ => unreachable!(),
            }
        }).collect();

        beams.sort();
        beams.dedup();

        println!("{:?} - {}", energised.iter().map(|row| row.iter().filter(|e| **e).count()).sum::<usize>(), beams.len());
    }

    // println!("{:?}", beams);
    // println!("{:?}", energised);
    for row in energised.iter() {
        println!("{}", row.iter().map(|e| if *e { "#" } else { "." }).collect::<Vec<_>>().join(""));
    }
    // println!("{:?}", energised.iter().map(|row| row.iter().filter(|e| **e).count()).sum::<usize>());

}
