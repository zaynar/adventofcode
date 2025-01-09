// Part 1: 16 mins
// Part 1+2: 28 mins

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Op {
    Add(u32),
    Mul(u32),
    Square,
}

fn run(title: &str, input: &str) {
    let mut data: Vec<(Vec<u32>, Op, u32, usize, usize, usize)> = input.split("\n\n").map(|m| {
        let lines = m.lines().collect_vec();
        let items = lines[1].strip_prefix("  Starting items: ").unwrap().split(", ").map(|n| n.parse().unwrap()).collect();
        let opstr = lines[2].strip_prefix("  Operation: new = old ").unwrap();
        let op = if opstr.starts_with("+") {
            Op::Add(opstr[2..].parse().unwrap())
        } else if opstr == "* old" {
            Op::Square
        } else {
            Op::Mul(opstr[2..].parse().unwrap())
        };
        let test = lines[3].strip_prefix("  Test: divisible by ").unwrap().parse().unwrap();
        let tt = lines[4].strip_prefix("    If true: throw to monkey ").unwrap().parse().unwrap();
        let tf = lines[5].strip_prefix("    If false: throw to monkey ").unwrap().parse().unwrap();
        (items, op, test, tt, tf, 0)
    }).collect_vec();

    // println!("{:?}", data);

    for round in 0..20 {
        for i in 0..data.len() {
            let (items, op, test, tt, tf, count) = data[i].clone();

            data[i].5 += items.len();
            for v in items {
                let v = match op {
                    Op::Add(n) => v + n,
                    Op::Mul(n) => v * n,
                    Op::Square => v * v,
                };
                let v = v / 3;
                if v % test == 0 {
                    data[tt].0.push(v);
                } else {
                    data[tf].0.push(v);
                }
            }
            data[i].0.clear();
        }

        // println!("{:?}", data);
    }

    println!("{:?}", data);

    let part1 = data.iter().map(|(items, op, test, tt, tf, count)| count).sorted().rev().collect_vec();

    println!("{} part 1: {}", title, part1[0] * part1[1]);
}

#[derive(Debug, Clone)]
struct Num {
    nmod: Vec<u32>,
}

impl Num {
    fn new(x: u32) -> Self {
        Num { nmod: (0..25).map(|m| x % m.max(1) as u32).collect() }
    }

    fn add(&self, y: u32) -> Self {
        // n mod m = x
        // (n + y) mod m = (x + y) mod m
        Num { nmod: self.nmod.iter().enumerate().map(|(m, x)| (x + y) % m.max(1) as u32).collect() }
    }

    fn mul(&self, y: u32) -> Self {
        Num { nmod: self.nmod.iter().enumerate().map(|(m, x)| (x * y) % m.max(1) as u32).collect() }
    }

    fn square(&self) -> Self {
        Num { nmod: self.nmod.iter().enumerate().map(|(m, x)| (x * x) % m.max(1) as u32).collect() }
    }

    fn is_div(&self, y: u32) -> bool {
        self.nmod[y as usize] == 0
    }
}
// PS: Should have just done this with basic arithmetic mod (2*3*5*7*11*13*17*19)

fn run2(title: &str, input: &str) {
    let mut data: Vec<(Vec<Num>, Op, u32, usize, usize, usize)> = input.split("\n\n").map(|m| {
        let lines = m.lines().collect_vec();
        let items = lines[1].strip_prefix("  Starting items: ").unwrap().split(", ").map(|n| Num::new(n.parse().unwrap())).collect();
        let opstr = lines[2].strip_prefix("  Operation: new = old ").unwrap();
        let op = if opstr.starts_with("+") {
            Op::Add(opstr[2..].parse().unwrap())
        } else if opstr == "* old" {
            Op::Square
        } else {
            Op::Mul(opstr[2..].parse().unwrap())
        };
        let test = lines[3].strip_prefix("  Test: divisible by ").unwrap().parse().unwrap();
        let tt = lines[4].strip_prefix("    If true: throw to monkey ").unwrap().parse().unwrap();
        let tf = lines[5].strip_prefix("    If false: throw to monkey ").unwrap().parse().unwrap();
        (items, op, test, tt, tf, 0)
    }).collect_vec();

    // println!("{:?}", data);

    // for round in 0..20 {
    for round in 0..10_000 {
            for i in 0..data.len() {
            let (items, op, test, tt, tf, count) = data[i].clone();

            data[i].5 += items.len();
            for v in items {
                let v = match op {
                    Op::Add(n) => v.add(n),
                    Op::Mul(n) => v.mul(n),
                    Op::Square => v.square(),
                };

                if v.is_div(test) {
                    data[tt].0.push(v);
                } else {
                    data[tf].0.push(v);
                }
            }
            data[i].0.clear();
        }

        // println!("{:?}", data.iter().map(|(items, op, test, tt, tf, count)| count).collect_vec());
    }

    // println!("{:?}", data);

    let part2 = data.iter().map(|(items, op, test, tt, tf, count)| count).sorted().rev().collect_vec();

    println!("{} part 2: {}", title, part2[0] * part2[1]);
}

const INPUT_DEMO: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("11/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("11/input.txt").unwrap());
}
