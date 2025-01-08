// Part 1: 16 mins
// Part 1+2: 20 mins

#[derive(Debug)]
enum Instr {
    Noop,
    Addx(i32),
}

fn run(title: &str, input: &str) {

    let mut x = 1;

    let mut part1 = 0;

    let mut cycle = 1;
    for line in input.lines() {

        let (instr, cyc) = if line == "noop" {
            (Instr::Noop, 1)
        } else {
            (Instr::Addx(line[5..].parse().unwrap()), 2)
        };

        for i in 0..cyc {
            if (cycle - 20) % 40 == 0 {
                println!("During {}th: {}", cycle, x);
                part1 += x * cycle;
            }
            cycle += 1;
        }

        match instr {
            Instr::Noop => (),
            Instr::Addx(n) => x += n,
        }
    }

    println!("{} part 1: {}", title, part1);
}

fn run2(title: &str, input: &str) {

    let mut x: i32 = 1;

    let mut out = String::new();

    let mut cycle = 0;
    for line in input.lines() {

        let (instr, cyc) = if line == "noop" {
            (Instr::Noop, 1)
        } else {
            (Instr::Addx(line[5..].parse().unwrap()), 2)
        };

        for i in 0..cyc {
            if x.abs_diff(cycle % 40) <= 1 {
                out += "#";
            } else {
                out += ".";
            }
            if cycle % 40 == 39 {
                out += "\n";
            }
            cycle += 1;
        }

        match instr {
            Instr::Noop => (),
            Instr::Addx(n) => x += n,
        }
    }

    println!("{}", out);
}

const INPUT_DEMO0: &str = "noop
addx 3
addx -5
";

const INPUT_DEMO: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

fn main() {
    // run("demo", INPUT_DEMO0);
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("10/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
