// Part 1: 9 mins
// Part 1+2: 10 mins

use itertools::Itertools;

fn dist(mut x: i32, mut y: i32) -> i32 {
    let mut steps = 0;
    while x > 0 && y > 0 {
        x -= 1;
        y -= 1;
        steps += 1;
    }
    while x < 0 && y < 0 {
        x += 1;
        y += 1;
        steps += 1;
    }
    steps += x.abs() + y.abs();

    steps

}

fn run(title: &str, input: &str) {
    let data: Vec<&str> = input.trim().split(",").collect_vec();

    let mut max_dist = 0;

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for step in &data {
        let (dx, dy) = match *step {
            "nw" => (-1, 0),
            "n" => (-1, -1),
            "ne" => (0, -1),
            "sw" => (0, 1),
            "s" => (1, 1),
            "se" => (1, 0),
            _ => panic!(),
        };

        x += dx;
        y += dy;

        max_dist = max_dist.max(dist(x, y));
    }

    println!("{} part 1: {}", title, dist(x, y));

    println!("{} part 2: {}", title, max_dist);
}

fn main() {
    run("demo", "ne,ne,ne");
    run("demo", "ne,ne,sw,sw");
    run("demo", "ne,ne,s,s");
    run("demo", "se,sw,se,sw,sw");
    run("input", &std::fs::read_to_string("11/input.txt").unwrap());
}
