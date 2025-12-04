// Part 1: 4 mins
// Part 1+2: 6 mins

fn run(title: &str, input: &str) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut part1 = 0;
    let mut part2 = 0;

    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] != '@' {
                continue;
            }

            let mut c = 0;
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if (dx, dy) == (0, 0) {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= w || ny < 0 || ny >= h {
                        continue;
                    }

                    if grid[ny as usize][nx as usize] == '@' {
                        c += 1;
                    }
                }
            }

            if c < 4 {
                part1 += 1;
            }
        }
    }

    loop {
        let mut dirty = false;
        for y in 0..h {
            for x in 0..w {
                if grid[y as usize][x as usize] != '@' {
                    continue;
                }

                let mut c = 0;
                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        if (dx, dy) == (0, 0) {
                            continue;
                        }
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx < 0 || nx >= w || ny < 0 || ny >= h {
                            continue;
                        }

                        if grid[ny as usize][nx as usize] == '@' {
                            c += 1;
                        }
                    }
                }

                if c < 4 {
                    part2 += 1;
                    grid[y as usize][x as usize] = '.';
                    dirty = true;
                }
            }
        }
        if !dirty {
            break;
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
