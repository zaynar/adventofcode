// Part 1: 16 mins
// Part 1+2: 19 mins

use std::collections::HashSet;

use itertools::Itertools;

fn run(title: &str, input: &str, limit: i32) {
    let data: Vec<(usize, i32, i32)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (x, y) = line.split_once(", ").unwrap();
            (i, x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect();

    let w = 400;
    let h = 400;
    let mut map = vec![usize::MAX; w * h];
    let mut inf = HashSet::new();
    inf.insert(usize::MAX);

    let mut part2 = 0;
    for y in 0..h {
        for x in 0..w {
            if data.iter().map(|(id, px, py)| (px - x as i32).abs() + (py - y as i32).abs()).sum::<i32>() < limit {
                part2 += 1;
            }

            let areas = data.iter().map(|(id, px, py)| ((px - x as i32).abs() + (py - y as i32).abs(), id)).sorted().collect_vec();
            if areas[0].0 != areas[1].0 {
                map[x + y*w] = *areas[0].1;
            }
            if x == 0 || y == 0 || x == w-1 || y == h-1 {
                inf.insert(*areas[0].1);
            }
        }
    }

    // for y in 0..h {
    //     for x in 0..w {
    //         if inf.contains(&map[x + y*w]) {
    //             print!(".");
    //         } else {
    //             print!("{}", ('A' as u8 + map[x + y*w] as u8) as char);
    //         }
    //     }
    //     println!();
    // }

    let counts = map.iter().counts();

    // println!("{:?}", counts);

    println!("{} part 1: {}", title, counts.iter().filter_map(|(k, v)| if inf.contains(k) { None } else { Some(v) }).max().unwrap());

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

fn main() {
    run("demo", INPUT_DEMO, 32);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap(), 10_000);
}
