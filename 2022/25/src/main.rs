// Part 1: 7 mins
// Part 1+2: 7 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let sum: i64 = input.lines().map(|line| {
        let mut n: i64 = 0;
        for c in line.chars() {
            n = n * 5;
            match c {
                '=' => n -= 2,
                '-' => n -= 1,
                '0' => n += 0,
                '1' => n += 1,
                '2' => n += 2,
                _ => panic!(),
            }
        }
        // println!("{:8} {}", line, n);
        n
    }).sum();

    println!("{} part 1: {}", title, sum);

    let mut ret = vec![];
    let mut n = sum;
    while n != 0 {
        let (d,c) = match n % 5 {
            0 => (0, '0'),
            1 => (1, '1'),
            2 => (2, '2'),
            3 => (-2, '='),
            4 => (-1, '-'),
            _ => panic!(),
        };
        n -= d;
        assert!(n % 5 == 0);
        ret.push(c);
        n /= 5;
    }

    println!("{} part 2: {}", title, ret.iter().rev().join(""));
}

const INPUT_DEMO: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("25/input.txt").unwrap());
}
