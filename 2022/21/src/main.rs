// Part 1: 9 mins
// Part 1+2: 33 mins

use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug)]
enum Node {
    Humn,
    Num(i64),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

fn eval(data: &HashMap<String, Op>, id: &str, cache: &mut HashMap<String, i64>) -> i64 {
    // if let Some(n) = cache.get(id) {
    //     return *n;
    // }

    let n: i64 = match &data[id] {
        Op::Num(n) => *n,
        Op::Add(a, b) => eval(data, &a, cache) + eval(data, &b, cache),
        Op::Sub(a, b) => eval(data, &a, cache) - eval(data, &b, cache),
        Op::Mul(a, b) => eval(data, &a, cache) * eval(data, &b, cache),
        Op::Div(a, b) => eval(data, &a, cache) / eval(data, &b, cache),
    };

    cache.insert(id.to_owned(), n);

    n
}

fn expand(data: &HashMap<String, Op>, id: &str) -> Node {
    if id == "humn" {
        return Node::Humn;
    }

    match &data[id] {
        Op::Num(n) => Node::Num(*n),
        Op::Add(a, b) => Node::Add(Box::new(expand(data, a)), Box::new(expand(data, b))),
        Op::Sub(a, b) => Node::Sub(Box::new(expand(data, a)), Box::new(expand(data, b))),
        Op::Mul(a, b) => Node::Mul(Box::new(expand(data, a)), Box::new(expand(data, b))),
        Op::Div(a, b) => Node::Div(Box::new(expand(data, a)), Box::new(expand(data, b))),
    }
}

fn simplify(node: &Node) -> Node {
    match node {
        Node::Humn => Node::Humn,
        Node::Num(n) => Node::Num(*n),
        Node::Add(a, b) => {
            let a = simplify(a);
            let b = simplify(b);
            match (&a, &b) {
                (Node::Num(a), Node::Num(b)) => Node::Num(a + b),
                _ => Node::Add(Box::new(a), Box::new(b))
            }
        }
        Node::Sub(a, b) => {
            let a = simplify(a);
            let b = simplify(b);
            match (&a, &b) {
                (Node::Num(a), Node::Num(b)) => Node::Num(a - b),
                _ => Node::Sub(Box::new(a), Box::new(b))
            }
        }
        Node::Mul(a, b) => {
            let a = simplify(a);
            let b = simplify(b);
            match (&a, &b) {
                (Node::Num(a), Node::Num(b)) => Node::Num(a * b),
                _ => Node::Mul(Box::new(a), Box::new(b))
            }
        }
        Node::Div(a, b) => {
            let a = simplify(a);
            let b = simplify(b);
            match (&a, &b) {
                (Node::Num(a), Node::Num(b)) => Node::Num(a / b),
                _ => Node::Div(Box::new(a), Box::new(b))
            }
        }
    }
}

fn equate(node: &Node, target: i64) -> i64 {
    match node {
        Node::Humn => return target,
        Node::Num(n) => panic!(),
        Node::Add(a, b) => {
            // a + b == target
            if let Node::Num(a) = **a {
                return equate(b, target - a);
            }
            if let Node::Num(b) = **b {
                return equate(a, target - b);
            }
        }
        Node::Sub(a, b) => {
            // a - b == target
            if let Node::Num(a) = **a {
                return equate(b, a - target);
            }
            if let Node::Num(b) = **b {
                return equate(a, target + b);
            }
        }
        Node::Mul(a, b) => {
            // a * b == target
            if let Node::Num(a) = **a {
                return equate(b, target / a);
            }
            if let Node::Num(b) = **b {
                return equate(a, target / b);
            }
        }
        Node::Div(a, b) => {
            // a / b == target
            if let Node::Num(a) = **a {
                return equate(b, a / target);
            }
            if let Node::Num(b) = **b {
                return equate(a, b * target);
            }
        }
    }

    panic!();
}

fn run(title: &str, input: &str) {
    let mut data: HashMap<String, Op> = HashMap::from_iter(
        input.lines()
        .map(|line| {
            let (id, out) = line.split_once(": ").unwrap();
            if let Ok(n) = out.parse::<i64>() {
                (id.to_owned(), Op::Num(n))
            } else {
                let a = out[0..4].to_owned();
                let b = out[7..11].to_owned();
                (id.to_owned(), match &out[5..6] {
                    "+" => Op::Add(a, b),
                    "-" => Op::Sub(a, b),
                    "*" => Op::Mul(a, b),
                    "/" => Op::Div(a, b),
                    _ => panic!()
                })
            }
        })
    );

    // println!("{:?}", data);

    // for (k, v) in &data {
    //     println!("{};", k);
    //     match v {
    //         Op::Num(n) => (),
    //         Op::Add(a, b) => println!("{} -> {}; {} -> {};", k, a, k, b),
    //         Op::Sub(a, b) => println!("{} -> {}; {} -> {};", k, a, k, b),
    //         Op::Mul(a, b) => println!("{} -> {}; {} -> {};", k, a, k, b),
    //         Op::Div(a, b) => println!("{} -> {}; {} -> {};", k, a, k, b),
    //     };
    // }

    let mut cache = HashMap::new();

    println!("{} part 1: {}", title, eval(&data, "root", &mut cache));

    match &data["root"] {
        Op::Add(lhs, rhs) => {
            let lhs = simplify(&expand(&data, lhs));
            let rhs = simplify(&expand(&data, rhs));
            println!("{:?}", lhs);
            println!("{:?}", rhs);
            if let Node::Num(rhs) = rhs {
                println!("{} part 2: {}", title, equate(&lhs, rhs));
            }
        }
        _ => panic!()
    };
}

const INPUT_DEMO: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("21/input.txt").unwrap());
}
