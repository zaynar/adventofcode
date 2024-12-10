// Part 1: 8 mins
// Part 1+2: 9 mins

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct Op {
    reg: String,
    op: String,
    n: i32,
    cr: String,
    cop: String,
    cn: i32,
}

fn cmp(a: i32, b: i32, op: &String) -> bool {
    match op.as_str() {
        "==" => a == b,
        "!=" => a != b,
        "<" => a < b,
        "<=" => a <= b,
        ">" => a > b,
        ">=" => a >= b,
        _ => panic!()
    }
}

fn run(title: &str, input: &str) {
    let data: Vec<Op> = input
        .lines()
        .map(|line| {
            let vs = line.split_whitespace().collect_vec();
            Op {
                reg: vs[0].to_owned(),
                op: vs[1].to_owned(),
                n: vs[2].parse().unwrap(),
                cr: vs[4].to_owned(),
                cop: vs[5].to_owned(),
                cn: vs[6].parse().unwrap(),
            }
        })
        .collect();

    // println!("{:?}", data);

    let mut regs = HashMap::new();
    let mut part2 = 0;
    for op in &data {
        // println!("{:?}", op);
        let r = regs.get(&op.cr).unwrap_or(&0);
        if cmp(*r, op.cn, &op.cop) {
            match op.op.as_str() {
                "inc" => *regs.entry(&op.reg).or_insert(0) += op.n,
                "dec" => *regs.entry(&op.reg).or_insert(0) -= op.n,
                _ => panic!()
            }
        }

        if !regs.is_empty() {
            part2 = part2.max(*regs.values().max().unwrap());
        }
    }

    println!("{} part 1: {}", title, regs.values().max().unwrap());

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
