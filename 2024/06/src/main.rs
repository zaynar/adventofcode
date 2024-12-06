use std::collections::HashSet;

fn walk(data: &Vec<Vec<bool>>, mut gx: i32, mut gy: i32) -> Option<usize> {
    let w = data[0].len() as i32;
    let h = data.len() as i32;

    let mut visited = HashSet::new();
    let mut visited_dir = HashSet::new();

    let mut dir = (0, -1);
    while 0 <= gx && gx < w && 0 <= gy && gy < h {
        if !visited_dir.insert((gx, gy, dir)) {
            return None;
        }

        visited.insert((gx, gy));

        let n = data.get((gy + dir.1) as usize).and_then(|r| r.get((gx + dir.0) as usize)).copied().unwrap_or(false);
        if n {
            dir = (-dir.1, dir.0);
        } else {
            gx += dir.0;
            gy += dir.1;
        }
    }

    // for y in 0..h {
    //     for x in 0..w {
    //         if visited.contains(&(x, y)) {
    //             print!("X");
    //         } else if data[y as usize][x as usize] {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    return Some(visited.len());
}

fn run(title: &str, input: &str) {

    let (mut gx, mut gy) = (0, 0);

    let data: Vec<Vec<bool>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(|(x, c)|
            match c {
                '.' => false,
                '#' => true,
                '^' => { (gx, gy) = (x as i32, y as i32); false },
                _ => panic!(),
            }
        ).collect()).collect();

    let w = data[0].len() as i32;
    let h = data.len() as i32;

    println!("{} part 1: {}", title, walk(&data, gx, gy).unwrap());

    let mut part2 = 0;
    for y in 0..h {
        for x in 0..w {
            if (x, y) != (gx, gy) && !data[y as usize][x as usize] {
                let mut data2 = data.clone();
                data2[y as usize][x as usize] = true;
                if walk(&data2, gx, gy).is_none() {
                    part2 += 1;
                }
            }
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
