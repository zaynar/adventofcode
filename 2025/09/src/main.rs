// Part 1: 4 mins
// Part 1+2: 21 mins

use itertools::Itertools;

fn area1(data: &Vec<(i64, i64)>, x0: i64, y0: i64, x1: i64, y1: i64) -> Option<i64> {
    Some((x1 - x0 + 1) * (y1 - y0 + 1))
}

fn isct(px0: i64, px1: i64, py: i64, x0: i64, y0: i64, x1: i64, y1: i64) -> bool {
    if px0 <= x0 && px1 <= x0 {
        return false;
    }

    if px0 >= x1 && px1 >= x1 {
        return false;
    }

    if py <= y0 || py >= y1 {
        return false;
    }

    return true;
}

fn area2(data: &Vec<(i64, i64)>, x0: i64, y0: i64, x1: i64, y1: i64) -> Option<i64> {
    for (p0, p1) in data.iter().circular_tuple_windows() {
        assert!(p0.0 == p1.0 || p0.1 == p1.1);
        if p0.1 == p1.1 {
            // Horizontal line
            if isct(p0.0, p1.0, p0.1, x0, y0, x1, y1) {
                return None;
            }
        } else {
            // Vertical line, so transpose the coords
            if isct(p0.1, p1.1, p0.0, y0, x0, y1, x1) {
                return None;
            }
        }
    }

    Some((x1 - x0 + 1) * (y1 - y0 + 1))
}

fn run(title: &str, input: &str) {
    let data: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    // let part1 = data.iter().tuple_combinations().map(|((x0, y0), (x1, y1))|
    //     ((x1 - x0).abs() + 1) * ((y1 - y0).abs() + 1)).max().unwrap();

    let part1 = data
        .iter()
        .tuple_combinations()
        .filter_map(|((x0, y0), (x1, y1))| {
            area1(&data, *x0.min(x1), *y0.min(y1), *x0.max(x1), *y0.max(y1))
        })
        .max()
        .unwrap();

    println!("{} part 1: {}", title, part1);

    let part2 = data
        .iter()
        .tuple_combinations()
        .filter_map(|((x0, y0), (x1, y1))| {
            area2(&data, *x0.min(x1), *y0.min(y1), *x0.max(x1), *y0.max(y1))
        })
        .max();

    println!("{} part 2: {:?}", title, part2);
}

const INPUT_DEMO: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
