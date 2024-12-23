// Part 1: 59 mins
// Part 2: 61 mins

use std::fmt;

use itertools::Itertools;

#[derive(Clone)]
enum Num {
    Literal(u32),
    Push,
    Pop,
    Comma,
    // Pair(Box<Num>, Box<Num>),
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Literal(x) => write!(f, "{}", x),
            // Num::Pair(a, b) => write!(f, "[{:?},{:?}]", a, b)
            Num::Push => write!(f, "["),
            Num::Pop => write!(f, "]"),
            Num::Comma => write!(f, ""),
        }
    }
}

fn printnum(num: &Vec<Num>) {
    for c in num {
        match c {
            Num::Literal(x) => print!("{}", x),
            Num::Push => print!("["),
            Num::Pop => print!("]"),
            Num::Comma => print!(","),
        }
    }
    println!();
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> Num
            = n:$(['0'..='9']+) { Num::Literal(n.parse().unwrap()) }

        rule push() -> Num
            = "[" { Num::Push }

        rule pop() -> Num
            = "]" { Num::Pop }

        pub rule num() -> Vec<Num>
            = (number() / push() / pop() / "," { Num::Comma })+
    }
}

fn explode(n: &Vec<Num>) -> Option<Vec<Num>> {
    let mut d = 0;
    for i in 0..n.len() {
        match n[i] {
            Num::Literal(_) => (),
            Num::Push => {
                if d == 4 {
                    let mut n = n.clone();

                    match (n[i + 1].clone(), n[i + 3].clone()) {
                        (Num::Literal(a), Num::Literal(b)) => {
                            for k in 0..5 {
                                n.remove(i);
                            }
                            n.insert(i, Num::Literal(0));
                            for k in (0..i).rev() {
                                if let Num::Literal(x) = n[k] {
                                    n[k] = Num::Literal(x + a);
                                    break;
                                }
                            }
                            for k in ((i+1)..n.len()) {
                                if let Num::Literal(x) = n[k] {
                                    n[k] = Num::Literal(x + b);
                                    break;
                                }
                            }
                            return Some(n);
                        }
                        _ => panic!()
                    }

                }
                d += 1;
            }
            Num::Pop => d -= 1,
            Num::Comma => (),
        }
    }
    return None;
}

fn split(n: &Vec<Num>) -> Option<Vec<Num>> {
    let mut d = 0;
    for i in 0..n.len() {
        match n[i] {
            Num::Literal(x) if x >= 10 => {
                let mut n = n.clone();
                n.remove(i);
                n.insert(i, Num::Pop);
                n.insert(i, Num::Literal((x + 1) / 2));
                n.insert(i, Num::Comma);
                n.insert(i, Num::Literal(x / 2));
                n.insert(i, Num::Push);
                return Some(n);
            }
            _ => (),
        }
    }
    return None;
}

fn mag(n: &Vec<Num>) -> u32 {
    let mut d = 0;
    let mut stack = vec![];
    for i in 0..n.len() {
        match n[i] {
            Num::Literal(x) => stack.push(x),
            Num::Push => d += 1,
            Num::Pop => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a*3 + b*2);
                d -= 1;
            }
            Num::Comma => (),
        }
    }
    return stack.pop().unwrap();
}

fn reduce(n: &Vec<Num>) -> Vec<Num> {
    let mut n = n.clone();
    loop {
        if let Some(m) = explode(&n) {
            n = m;
            continue;
        }
        if let Some(m) = split(&n) {
            n = m;
            continue;
        }
        return n;
    }
}

fn run(title: &str, input: &str) {
    let mut sum: Option<Vec<Num>> = None;
    for line in input.lines() {
        let n = input_parser::num(line).unwrap();
        sum = if let Some(s) = &sum {
            let mut v = vec![Num::Push];
            v.append(&mut s.clone());
            v.push(Num::Comma);
            v.append(&mut n.clone());
            v.push(Num::Pop);
            Some(reduce(&v))
        } else {
            Some(n)
        };
        // printnum(&sum.clone().unwrap());
    }

    println!("{} part 1: {}", title, mag(&sum.unwrap()));

    let mut part2 = 0;
    for a in input.lines() {
        for b in input.lines() {
            if a == b { continue; }

            let a = input_parser::num(a).unwrap();
            let b = input_parser::num(b).unwrap();
            let mut v = vec![Num::Push];
            v.append(&mut a.clone());
            v.push(Num::Comma);
            v.append(&mut b.clone());
            v.push(Num::Pop);
            let m = mag(&reduce(&v));
            part2 = part2.max(m);
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

fn main() {
    run("demo", INPUT_DEMO);
    // run("demo", "[[[[[9,8],1],2],3],4]");
    // run("demo", "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    run("input", &std::fs::read_to_string("18/input.txt").unwrap());
}
