// Part 1: 7 mins
// Part 1+2: 9 mins

fn run(title: &str, input: &str) {
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (1, 0);

    for line in input.lines() {
        let (c, n) = (line.chars().nth(0).unwrap(), line[1..].parse::<i32>().unwrap());
        // println!("{} {} {},{} {},{}", c, n, x, y, dx, dy);
        match c {
            'N' => {
                y -= n;
            }
            'S' => {
                y += n;
            }
            'W' => {
                x -= n;
            }
            'E' => {
                x += n;
            }
            'L' => {
                for i in 0..n/90 {
                    (dx, dy) = (dy, -dx)
                }
            }
            'R' => {
                for i in 0..n/90 {
                    (dx, dy) = (-dy, dx)
                }
            }
            'F' => {
                x += dx * n;
                y += dy * n;
            }
            _ => panic!("{}", c)
        }
    }

    println!("{} part 1: {}", title, x.abs() + y.abs());
}

fn run2(title: &str, input: &str) {
    let (mut x, mut y) = (0, 0);
    let (mut wx, mut wy) = (10, -1);

    for line in input.lines() {
        let (c, n) = (line.chars().nth(0).unwrap(), line[1..].parse::<i32>().unwrap());
        // println!("{} {} {},{} {},{}", c, n, x, y, dx, dy);
        match c {
            'N' => {
                wy -= n;
            }
            'S' => {
                wy += n;
            }
            'W' => {
                wx -= n;
            }
            'E' => {
                wx += n;
            }
            'L' => {
                for i in 0..n/90 {
                    (wx, wy) = (wy, -wx)
                }
            }
            'R' => {
                for i in 0..n/90 {
                    (wx, wy) = (-wy, wx)
                }
            }
            'F' => {
                x += wx * n;
                y += wy * n;
            }
            _ => panic!("{}", c)
        }
    }

    // 990 too low
    println!("{} part 1: {}", title, x.abs() + y.abs());

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "F10
N3
F7
R90
F11
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("12/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
