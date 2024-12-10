// Part 1: 3 mins
// Part 1+2: 6 mins

fn run(title: &str, input: &str) {
    let fa: u64 = 16807;
    let fb: u64 = 48271;
    let d = 2147483647;

    // Generator A starts with 679
    // Generator B starts with 771
    let mut a = 679;
    let mut b = 771;

    // let mut a = 65;
    // let mut b = 8921;

    let mut part1 = 0;

    for i in 0..40_000_000 {
        a = (a * fa) % d;
        b = (b * fb) % d;
        if (a & 0xffff) == (b & 0xffff) {
            part1 += 1;
        }
        // println!("{} {}", a, b);
    }

    println!("{} part 1: {}", title, part1);

    let mut a = 679;
    let mut b = 771;
    // let mut a = 65;
    // let mut b = 8921;

    let mut part2 = 0;

    for i in 0..5_000_000 {
        a = (a * fa) % d;
        while a % 4 != 0 {
            a = (a * fa) % d;
        }
        b = (b * fb) % d;
        while b % 8 != 0 {
            b = (b * fb) % d;
        }
        if (a & 0xffff) == (b & 0xffff) {
            part2 += 1;
        }
        // println!("{} {}", a, b);
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "";

fn main() {
    run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("15/input.txt").unwrap());
}
