// Part 1: 10 mins
// Part 2: 25 mins

use std::collections::{HashSet, VecDeque};

fn run(title: &str, input: &str) {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut done: HashSet<(isize, isize)> = HashSet::new();

    let h = data.len() as isize;
    let w = data[0].len() as isize;

    let mut part1 = 0;
    let mut part2 = 0;

    for y in 0..h {
        for x in 0..w {

            if done.contains(&(x as isize, y as isize)) { continue; }

            let mut open = VecDeque::new();
            open.push_back((x as isize, y as isize));
            let mut seen: HashSet<(isize, isize)> = HashSet::new();

            let mut area = 0;
            let mut perim = 0;

            while let Some((x, y)) = open.pop_back() {
                done.insert((x, y));
                if !seen.insert((x, y)) {
                    continue;
                }

                area += 1;

                for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as isize + d.0;
                    let ny = y as isize + d.1;
                    if 0 <= nx && nx < w && 0 <= ny && ny < h {
                        if data[ny as usize][nx as usize] == data[y as usize][x as usize] {
                            open.push_back((nx, ny));
                        } else {
                            perim += 1;
                        }
                    } else {
                        perim += 1;
                    }
                }
            }

            // println!("{} {} {} a={} p={}", x, y, data[y][x], area, perim);
            part1 += area * perim;

            let mut hsides = 0;
            let mut vsides = 0;
            for y in -1..=h {
                let mut p = (false, false);
                for x in 0..w {
                    let c = (seen.contains(&(x, y)), seen.contains(&(x, y+1)));
                    // if area == 10 && x > 4 && y < 8 { println!(" h {} {} {:?} {:?} {}", x, y, c, p, hsides); }
                    if c != p && (c.0 != c.1) {
                        hsides += 1;
                    }
                    p = c;
                }
            }
            for x in -1..=w {
                let mut p = (false, false);
                for y in 0..h {
                    let c = (seen.contains(&(x, y)), seen.contains(&(x+1, y)));
                    // if area == 10 && x > 4 && y < 8 { println!(" v {} {} {:?} {:?} {}", x, y, c, p, vsides); }
                    if c != p && (c.0 != c.1) {
                        vsides += 1;
                    }
                    p = c;
                }
            }

            part2 += area * (hsides + vsides);

            // println!("{} {} {} a={} p={} s={},{}", x, y, data[y as usize][x as usize], area, perim, hsides, vsides);

        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
