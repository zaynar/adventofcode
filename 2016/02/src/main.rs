use std::collections::HashMap;

fn run(title: &str, input: &str) {
    let mut n: i32 = 5;
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => if n >= 4 { n -= 3; }
                'D' => if n <= 6 { n += 3; }
                'L' => if ![1,4,7].contains(&n) { n -= 1; }
                'R' => if ![3,6,9].contains(&n) { n += 1; }
                _ => panic!()
            }
        }

        print!("{}", n);
    }
    println!();

    let map = HashMap::from([
        ('U', HashMap::from([(3,1), (6,2), (7,3), (8,4), (10,6), (11,7), (12,8), (13,11)])),
        ('D', HashMap::from([(3,1), (6,2), (7,3), (8,4), (10,6), (11,7), (12,8), (13,11)].map(|(a,b)| (b,a)))),
        ('L', HashMap::from([(3,2), (4,3), (6,5), (7,6), (8,7), (9,8), (11,10), (12,11)])),
        ('R', HashMap::from([(3,2), (4,3), (6,5), (7,6), (8,7), (9,8), (11,10), (12,11)].map(|(a,b)| (b,a)))),
    ]);

    let mut n: i32 = 5;
    for line in input.lines() {
        for c in line.chars() {
            n = map[&c].get(&n).copied().unwrap_or(n);
        }

        print!("{:X}", n);
    }
    println!();
}

const INPUT_DEMO: &str = "ULL
RRDDD
LURDL
UUUUD
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
