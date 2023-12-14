use std::{fs, collections::HashMap};

fn part1(grid: &Vec<Vec<char>>) {
    let mut grid = grid.clone();

    for _i in 0..grid.len() {
        for y in 0..(grid.len()-1) {
            for x in 0..grid[0].len() {
                if grid[y][x] == '.' {
                    if grid[y+1][x] == 'O' {
                        grid[y][x] = 'O';
                        grid[y+1][x] = '.';
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for (i, line) in grid.iter().enumerate() {
        let w = grid.len() - i;
        println!("{} {}", line.iter().collect::<String>(), w);
        sum += line.iter().filter(|&&c| c == 'O').count() * w;
    }

    println!("Answer 1: {}", sum);
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut grid = grid.clone();

    let mut seen = HashMap::new();

    for j in 0..1000 {

        if let Some(prev) = seen.get(&grid) {
            println!("Cycle: {} = {}", j, prev);
            if (1000000000 - j) % (j - prev) == 0 {
                break;
            }
        }
        seen.insert(grid.clone(), j);

        for _i in 0..grid.len() {
            for y in 0..(grid.len()-1) {
                for x in 0..grid[0].len() {
                    if grid[y][x] == '.' {
                        if grid[y+1][x] == 'O' {
                            grid[y][x] = 'O';
                            grid[y+1][x] = '.';
                        }
                    }
                }
            }
        }

        for _i in 0..grid.len() {
            for y in 0..grid.len() {
                for x in 0..(grid[0].len()-1) {
                    if grid[y][x] == '.' {
                        if grid[y][x+1] == 'O' {
                            grid[y][x] = 'O';
                            grid[y][x+1] = '.';
                        }
                    }
                }
            }
        }

        for _i in 0..grid.len() {
            for y in 1..grid.len() {
                for x in 0..grid[0].len() {
                    if grid[y][x] == '.' {
                        if grid[y-1][x] == 'O' {
                            grid[y][x] = 'O';
                            grid[y-1][x] = '.';
                        }
                    }
                }
            }
        }

        for _i in 0..grid.len() {
            for y in 0..grid.len() {
                for x in 1..grid[0].len() {
                    if grid[y][x] == '.' {
                        if grid[y][x-1] == 'O' {
                            grid[y][x] = 'O';
                            grid[y][x-1] = '.';
                        }
                    }
                }
            }
        }

        // for (i, line) in grid.iter().enumerate() {
        //     println!("{}", line.iter().collect::<String>());
        // }
        // println!("");
    }

    let mut sum = 0;
    for (i, line) in grid.iter().enumerate() {
        let w = grid.len() - i;
        println!("{} {}", line.iter().collect::<String>(), w);
        sum += line.iter().filter(|&&c| c == 'O').count() * w;
    }

    println!("Answer 2: {}", sum);
}

fn main() {
    let grid: Vec<Vec<char>> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    part1(&grid);
    part2(&grid);
}