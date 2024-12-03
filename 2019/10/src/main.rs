extern crate nalgebra as na;
use core::f64;

use na::Vector2;

fn is_on_line(a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64>) -> bool {
    (a - c).perp(&(b - c)) == 0.0 && (a - c).norm_squared() <= (a - b).norm_squared() && (b - c).norm_squared() <= (b - a).norm_squared()
}

fn run(title: &str, input: &str) {
    let data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let w = data[0].len();
    let h = data.len();

    let mut asteroids = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if data[y][x] {
                asteroids.push(na::Vector2::new(x as f64, y as f64));
            }
        }
    }

    println!("{:?}", data);

    let mut max_visible = (0, 0, 0);
    for &a in &asteroids {
        let mut visible = 0;
        for &b in &asteroids {
            if b == a {
                continue;
            }

            if !asteroids.iter().any(|&c|
                c != a && c != b && is_on_line(a, b, c)
            ) {
                visible += 1;
            }

        }
        // println!("{:?} {}", (a.x, a.y), visible);
        max_visible = max_visible.max((visible, a.x as i32, a.y as i32));
    }

    println!("{} part 1: {:?}", title, max_visible);
}

fn run2(title: &str, input: &str, station: (i32, i32)) {
    let data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let w = data[0].len();
    let h = data.len();

    let mut asteroids = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if data[y][x] {
                let angle = f64::consts::PI/2.0 - f64::atan2(-(y as i32 - station.1) as f64, (x as i32 - station.0) as f64);
                let angle = (angle + f64::consts::PI*2.0) % (f64::consts::PI*2.0);
                let dist2 = (x as i32 - station.0).pow(2) + (y as i32 - station.1).pow(2);
                if dist2 == 0 {
                    continue;
                }
                asteroids.push((angle, dist2, (x, y), false));
            }
        }
    }

    asteroids.sort_by(|(a0, d0, _, _), (a1, d1, _, _)| a0.partial_cmp(a1).unwrap().then(d0.cmp(d1)));

    // for a in &asteroids {
    //     println!("{:?}", a);
    // }

    let mut laser = -1.0;
    let mut count = 0;
    for i in 0..400 {
        if let Some(target) = asteroids.iter_mut().filter(|(a, _, _, destroyed)| !destroyed && *a > laser).nth(0) {
            println!("TARGET {}th: {:?}", count+1, target);
            target.3 = true;
            laser = target.0;
            count += 1;
        } else {
            laser = -1.0;
        }
    }
}


const INPUT_DEMO: &str = ".#..#
.....
#####
....#
...##
";

const INPUT_DEMO2: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";

fn main() {
    // run("demo 1", INPUT_DEMO);
    // run("demo 2", INPUT_DEMO2);

//     run2("demo 3", ".#....#####...#..
// ##...##.#####..##
// ##...#...#.#####.
// ..#.........###..
// ..#.#.....#....##
// ", (8, 3));

//     run2("demo 3", "......#####......
// .................
// .................
// .................
// .................
// ", (8, 3));

    // run2("demo 2", INPUT_DEMO2, (11, 13));

    // run("input", &std::fs::read_to_string("10/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("10/input.txt").unwrap(), (17, 22));
}
