// Part 1: 22 mins
// Part 1+2: 26 mins

use std::collections::HashSet;

use itertools::Itertools;

/*

  0123
   #
  #SB
   #

*/

// [x0, x1)
fn part1(data: &Vec<(i32, i32, i32, i32)>, x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
    if x0 == x1 || y0 == y1 {
        return 0;
    }

    // println!("{} {}", x0, x1);

    let mut partial = false;

    for (sx, sy, bx, by) in data {
        let r = sx.abs_diff(*bx) + sy.abs_diff(*by);

        if sx.abs_diff(x0) + sy.abs_diff(y0) <= r &&
           sx.abs_diff(x1 - 1) + sy.abs_diff(y0) <= r &&
           sx.abs_diff(x0) + sy.abs_diff(y1 - 1) <= r &&
           sx.abs_diff(x1 - 1) + sy.abs_diff(y1 - 1) <= r
           {
            // Fully contained in this diamond
            // println!("# {}..{}", x0, x1);
            return x1 - x0;
        }

        if !(x1 <= sx - r as i32 || x0 > sx + r as i32) {
            partial = true;
        }

    }

    if !partial {
        return 0;
    }

    if x1 - x0 == 1 {
        return 0;
    }

    return part1(data, x0, (x0+x1) / 2, y0, y1) + part1(data, (x0+x1) / 2, x1, y0, y1);
}

fn part2(data: &Vec<(i32, i32, i32, i32)>, x0: i32, x1: i32, y0: i32, y1: i32) {
    if x0 == x1 || y0 == y1 {
        return;
    }

    // println!("{} {}", x0, x1);

    let mut partial = false;

    for (sx, sy, bx, by) in data {
        let r = sx.abs_diff(*bx) + sy.abs_diff(*by);

        if sx.abs_diff(x0) + sy.abs_diff(y0) <= r &&
           sx.abs_diff(x1 - 1) + sy.abs_diff(y0) <= r &&
           sx.abs_diff(x0) + sy.abs_diff(y1 - 1) <= r &&
           sx.abs_diff(x1 - 1) + sy.abs_diff(y1 - 1) <= r
           {
            // Fully contained in this diamond
            return;
        }

        if !(x1 <= sx - r as i32 || x0 > sx + r as i32) {
            partial = true;
        }

    }

    if !partial || (x1 - x0 == 1 && y1 - y0 == 1) {
        println!("part 2 {} {} {}", x0, y0, x0 as u64*4000000 + y0 as u64);
        return;
    }

    part2(data, x0, (x0+x1) / 2, y0, (y0+y1)/2);
    part2(data, (x0+x1) / 2, x1, y0, (y0+y1)/2);
    part2(data, x0, (x0+x1) / 2, (y0+y1)/2, y1);
    part2(data, (x0+x1) / 2, x1, (y0+y1)/2, y1);
}

fn run(title: &str, input: &str, test_y: i32) {
    let data: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            line.split(|p| "=,:".contains(p)).filter_map(|n| n.parse::<i32>().ok()).collect_tuple().unwrap()
        })
        .collect();

    println!("{:?}", data);

    // Each line defines a diamond
    // We want to compute the union of all diamonds
    // ...minus the actual beacons

    let p1beacons = HashSet::<(i32, i32)>::from_iter(data.iter().filter_map(|(a,b,c,d)| if *d == test_y { Some((*c,*d)) } else { None })).len();

    println!("{} part 1: {}", title, part1(&data, -10_000_000, 10_000_000, test_y, test_y + 1) - p1beacons as i32);
    // println!("{} part 1: {}", title, part1(&data, -10, 30, test_y, test_y + 1) - p1beacons as i32);

    // println!("{} part 2: {}", title, "TODO");
    // part2(&data, 0, 20+1, 0, 20+1);
    part2(&data, 0, 4000000+1, 0, 4000000+1);
}

const INPUT_DEMO: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

fn main() {
    // run("demo", INPUT_DEMO, 10);
    run("input", &std::fs::read_to_string("15/input.txt").unwrap(), 2_000_000);
}
