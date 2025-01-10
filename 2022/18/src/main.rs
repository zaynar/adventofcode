// Part 1: 5 mins
// Part 1+2: 9 mins

use std::collections::HashSet;

use itertools::Itertools;

fn exposed(cubes: &HashSet<(i32, i32, i32)>, x: i32, y: i32, z: i32) -> bool {

    let mut open = vec![(x, y, z)];
    let mut visited = HashSet::new();

    while let Some((x, y, z)) = open.pop() {

        if !visited.insert((x, y, z)) {
            continue;
        }

        if cubes.contains(&(x, y, z)) {
            continue;
        }
        if x < 0 || y < 0 || z < 0 || x > 25 || y > 25 || z > 25 {
            return true;
        }

        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx as i32).abs() + (dy as i32).abs() + (dz as i32).abs() == 1 {
                        open.push((x + dx, y + dy, z + dz));
                    }
                }
            }
        }
    }

    return false;
}

fn run(title: &str, input: &str) {

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for line in input.lines() {
        let (x, y, z) = line.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap();
        cubes.insert((x, y, z));
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for (x, y, z) in &cubes {
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx as i32).abs() + (dy as i32).abs() + (dz as i32).abs() == 1 {
                        if !cubes.contains(&(x + dx, y + dy, z + dz)) {
                            part1 += 1;

                            if exposed(&cubes, x + dx, y + dy, z + dz) {
                                part2 += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap());
}
