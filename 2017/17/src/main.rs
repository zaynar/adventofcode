// Part 1: 5 mins
// Part 1+2: 8 mins

use std::collections::VecDeque;

fn run(title: &str, input: usize) {
    let mut buf = Vec::new();
    buf.push(0);

    for i in 0..2017 {
        let len = buf.len();
        buf.rotate_left(input % len);
        buf.push(i + 1);
        // println!("{:?}", buf);
    }

    println!("{} part 1: {}", title, buf[0]);

    let mut buf = VecDeque::new();
    buf.push_back(0);

    for i in 0..50_000_000 {
        if i % 1_000_000 == 0 { println!("{}", i)};
        let len = buf.len();
        buf.rotate_left(input % len);
        buf.push_back(i + 1);
        // println!("{:?}", buf);
    }

    let p = buf.iter().position(|n| *n == 0).unwrap();
    println!("{} part 2: {}", title, buf[p + 1]);
}

const INPUT_DEMO: &str = "";

fn main() {
    // run("demo", 3);
    run("input", 312);
}
