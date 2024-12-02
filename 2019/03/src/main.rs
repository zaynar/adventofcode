use std::collections::HashMap;

fn evaluate(steps: &Vec<(char, i32)>) -> Vec<(i32, i32, i32, i32)> {
    let mut x = 0;
    let mut y = 0;

    let mut lines = Vec::new();
    for (dir, dist) in steps {
        let (nx, ny) = match dir {
            'L' => (x - dist, y),
            'R' => (x + dist, y),
            'U' => (x, y - dist),
            'D' => (x, y + dist),
            _ => panic!("Invalid dir"),
        };
        lines.push((x, y, nx, ny));
        (x, y) = (nx, ny);
    }
    lines
}

fn intersect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> Option<(i32, i32)> {
    let axmin = a.0.min(a.2);
    let axmax = a.0.max(a.2);
    let bxmin = b.0.min(b.2);
    let bxmax = b.0.max(b.2);
    let aymin = a.1.min(a.3);
    let aymax = a.1.max(a.3);
    let bymin = b.1.min(b.3);
    let bymax = b.1.max(b.3);

    // println!("{:?} {:?}", a, b);
    if axmin == axmax && bymin == bymax {
        // println!("{:?} {:?}", a, b);
        // a vertical, b horizontal
        if bxmin <= axmin && axmin <= bxmax && aymin <= bymin && bymin <= aymax {
            return Some((axmin, bymin));
        }
    }

    if b.0 == b.2 && a.1 == a.3 {
        // a horizontal, b vertical
        if axmin <= bxmin && bxmin <= axmax && bymin <= aymin && aymin <= bymax {
            return Some((bxmin, aymin));
        }
    }

    None
}

fn run(title: &str, input: &str) {
    let data: Vec<Vec<(char, i32)>> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| (v.chars().next().unwrap(), str::parse(&v[1..]).unwrap()))
                .collect()
        })
        .collect();

    // println!("{:?}", data);

    let lines0 = evaluate(&data[0]);
    let lines1 = evaluate(&data[1]);
    let mut isct = Vec::new();
    let mut isct2 = Vec::new();

    let mut steps0 = 0;
    for l0 in &lines0 {
        let mut steps1 = 0;
        for l1 in &lines1 {
            if let Some((x, y)) = intersect(*l0, *l1) {
                let dist = x.abs() + y.abs();
                if dist > 0 {
                    isct.push(dist);
                }

                let d0 = (x - l0.0).abs() + (y - l0.1).abs();
                let d1 = (x - l1.0).abs() + (y - l1.1).abs();
                let dist2 = steps0 + d0 + steps1 + d1;
                if dist2 > 0 {
                    isct2.push(dist2);
                }
            }
            steps1 += (l1.0 - l1.2).abs() + (l1.1 - l1.3).abs();
        }
        steps0 += (l0.0 - l0.2).abs() + (l0.1 - l0.3).abs();
    }

    println!("{} part 1: {}", title, isct.iter().min().unwrap());

    println!("{} part 2: {}", title, isct2.iter().min().unwrap());
}

const INPUT_DEMO: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
