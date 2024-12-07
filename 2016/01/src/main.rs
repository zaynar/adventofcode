use std::collections::HashSet;

fn run(title: &str, input: &str) {
    let mut dir = (0, -1);
    let (mut x, mut y) = (0i32, 0i32);

    let mut seen = HashSet::new();

    seen.insert((x, y));

    for d in input.trim().split(", ") {
        if d.starts_with("R") {
            dir = (-dir.1, dir.0);
        } else {
            dir = (dir.1, -dir.0);
        }
        let dist: i32 = d[1..].parse().unwrap();
        // println!("{} {:?} {}", d, dir, dist);

        for i in 0..dist {
            x += dir.0;
            y += dir.1;

            if !seen.insert((x, y)) {
                println!("{} part 2: {}", title, x.abs() + y.abs());
                return;
            }
        }
    }

    println!("{} part 1: {}", title, x.abs() + y.abs());
}

fn main() {
    run("demo", "R2, L3");
    run("demo", "R2, R2, R2");
    run("demo", "R5, L5, R5, R3");
    run("demo", "R8, R4, R4, R8");
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
