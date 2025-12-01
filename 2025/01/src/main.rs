// Part 1: 5 mins
// Part 1+2: 7 mins

fn run(title: &str, input: &str) {
    let mut pos: i32 = 50;
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let n = if let Some(n) = line.strip_prefix("L") {
            -n.parse::<i32>().unwrap()
        } else if let Some(n) = line.strip_prefix("R") {
            n.parse::<i32>().unwrap()
        } else {
            panic!();
        };

        for _ in 0..n.abs() {
            pos += n.signum();

            if pos.rem_euclid(100) == 0 {
                part2 += 1;
            }

        }

        if pos.rem_euclid(100) == 0 {
            part1 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
