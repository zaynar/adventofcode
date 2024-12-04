use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
enum Step {
    Reverse,
    Increment(i64),
    Cut(i64),
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i64
            = n:$("-"? ['0'..='9']+) {? n.parse().or(Err("number")) }

        rule line() -> Step
            = "deal into new stack\n" { Step::Reverse }
            / "deal with increment " n:number() "\n" { Step::Increment(n) }
            / "cut " n:number() "\n" { Step::Cut(n) }

        pub rule file() -> Vec<Step>
            = line()+
    }
}

fn inverse(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let q = r / newr;
        (t, newt) = (newt, t - q * newt);
        (r, newr) = (newr, r - q * newr);
    }

    assert!(r <= 1);
    if t < 0 {
        t += n;
    }

    t
}

fn run(title: &str, input: &str, count: i64) {
    /*
    Operations:

    Reverse stack
    Rotate N cards from top of stack to bottom (N can be negative)
    out[i*N % D] = in[i]

     */

    let data = input_parser::file(input).unwrap();

    let mut deck = (0..count).collect_vec();

    for step in &data {
        match step {
            Step::Reverse => deck.reverse(),
            Step::Increment(n) => {
                // let mut new_deck = deck.clone();
                // for i in 0..count {
                //     new_deck[((i * n) % count) as usize] = deck[i as usize];
                // }
                // deck = new_deck;
                let inv = inverse(*n, count);
                deck = (0..count).map(|i| deck[((i * inv) % count) as usize]).collect();
            }
            Step::Cut(n) => {
                if *n > 0 {
                    deck.rotate_left(*n as usize)
                } else {
                    deck.rotate_right((-*n) as usize)
                }
            }
        }
    }

    if count < 100 {
        println!("{:?}", deck);
    } else {
        println!(
            "{:?}",
            deck.iter().find_position(|c| **c == 2019).unwrap().0
        );
    }
}

fn card_at_pos(steps: &Vec<Step>, count: i64, mut pos: i64) -> i64 {
    for step in steps.iter().rev() {
        pos = match step {
            Step::Reverse => count - 1 - pos,
            Step::Increment(n) => {
                let inv = inverse(*n, count);
                ((pos as i128 * inv as i128) % count as i128) as i64
            }
            Step::Cut(n) => (pos + count + n) % count
        };
    }
    pos
}

#[derive(Debug, Clone)]
enum Expr {
    Pos,
    Num(i128),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn expand_str(steps: &Vec<Step>, count: i64, mut pos: String) -> String {
    for step in steps.iter().rev() {
        pos = match step {
            Step::Reverse => format!("(-1 - {})", pos),
            Step::Increment(n) => {
                let inv = inverse(*n, count);
                format!("({} * {})", pos, inv)
            }
            Step::Cut(n) => format!("({} + {})", pos, n)
        };
    }
    pos
}

fn expand(steps: &Vec<Step>, count: i64) -> Expr {
    let mut pos = Expr::Pos;
    for step in steps.iter().rev() {
        pos = match step {
            Step::Reverse => Expr::Neg(Box::new(Expr::Add(Box::new(Expr::Num(1)), Box::new(pos)))),
            Step::Increment(n) => {
                let inv = inverse(*n, count);
                Expr::Mul(Box::new(pos), Box::new(Expr::Num(inv as i128)))
            }
            Step::Cut(n) => Expr::Add(Box::new(pos), Box::new(Expr::Num(*n as i128)))
        };
    }
    pos
}

/*
NOTE: This is probably very silly, we could represent everything as pos*a+b and just apply all the steps within that representation
*/

fn simplify(expr: Expr, m: i128) -> Expr {
    // ((c+d) * b) => (c*b + d*b)
    if let Expr::Mul(a, b) = &expr {
        if let Expr::Num(b) = **b {
            if let Expr::Add(c, d) = &**a {
                if let Expr::Num(d) = **d {
                    return simplify(Expr::Add(
                        Box::new(simplify(Expr::Mul(c.clone(), Box::new(Expr::Num(b))), m)),
                        Box::new(Expr::Num(modulo(d * b, m))),
                    ), m);
                }
            }
        }
    }

    // ((c+d) + b) => (c + d+b)
    if let Expr::Add(a, b) = &expr {
        if let Expr::Num(b) = **b {
            if let Expr::Add(c, d) = &**a {
                if let Expr::Num(d) = **d {
                    return simplify(Expr::Add(
                        c.clone(),
                        Box::new(Expr::Num(modulo(d + b, m))),
                    ), m);
                }
            }
        }
    }

    // (b + (c+d)) => (c + d+b)
    if let Expr::Add(b, a) = &expr {
        if let Expr::Num(b) = **b {
            if let Expr::Add(c, d) = &**a {
                if let Expr::Num(d) = **d {
                    return simplify(Expr::Add(
                        c.clone(),
                        Box::new(Expr::Num(modulo(d + b, m))),
                    ), m);
                }
            }
        }
    }

    // ((c*d) * b) => (c * d*b)
    if let Expr::Mul(a, b) = &expr {
        if let Expr::Num(b) = **b {
            if let Expr::Mul(c, d) = &**a {
                if let Expr::Num(d) = **d {
                    return simplify(Expr::Mul(
                        c.clone(),
                        Box::new(Expr::Num(modulo(d * b, m))),
                    ), m);
                }
            }
        }
    }

    if let Expr::Add(a, b) = &expr {
        if let Expr::Pos = **a {
            if let Expr::Add(x, y) = &**b {

            }
        }
    }

    // -(a + b) => (-a + -b)
    if let Expr::Neg(a) = &expr {
        if let Expr::Add(b, c) = &**a {
            return simplify(Expr::Add(
                Box::new(Expr::Neg(b.clone())),
                Box::new(Expr::Neg(c.clone())),
            ), m);
        }
    }

    // -(b * c) => (b * -c)
    if let Expr::Neg(a) = &expr {
        if let Expr::Mul(b, c) = &**a {
            return simplify(Expr::Mul(
                b.clone(),
                Box::new(Expr::Neg(c.clone())),
            ), m);
        }
    }

    // --a => a
    if let Expr::Neg(a) = &expr {
        if let Expr::Neg(b) = &**a {
            return (**b).clone();
        }
    }

    if let Expr::Neg(a) = &expr {
        if let Expr::Num(b) = &**a {
            return Expr::Num(modulo(-b, m))
        }
    }

    match expr {
        Expr::Pos => Expr::Pos,
        Expr::Num(n) => Expr::Num(modulo(n, m)),
        Expr::Neg(a) => Expr::Neg(Box::new(simplify(*a, m))),
        Expr::Add(a, b) => Expr::Add(Box::new(simplify(*a, m)), Box::new(simplify(*b, m))),
        Expr::Mul(a, b) => Expr::Mul(Box::new(simplify(*a, m)), Box::new(simplify(*b, m))),
    }
}

fn evaluate(expr: &Expr, m: i128, pos: i128) -> i128 {
    match expr {
        Expr::Pos => pos,
        Expr::Num(n) => modulo(*n, m),
        Expr::Neg(a) => modulo(-evaluate(a, m, pos), m),
        Expr::Add(a, b) => modulo(evaluate(a, m, pos) + evaluate(b, m, pos), m),
        Expr::Mul(a, b) => modulo(evaluate(a, m, pos) * evaluate(b, m, pos), m),
    }
}

fn modulo(a: i128, m: i128) -> i128 {
    (a + m) % m
}

fn run2(title: &str, input: &str, count: i64) {
    /*
    Operations:

    Reverse stack
    Rotate N cards from top of stack to bottom (N can be negative)
    out[i*N % D] = in[i]

     */

    let steps = input_parser::file(input).unwrap();

    let mut deck = (0..count).map(|i| card_at_pos(&steps, count, i)).collect_vec();

    if count < 100 {
        println!("{} part 1: {:?}", title, deck);
    } else {
        println!(
            "{} part 1: {:?}",
            title,
            deck.iter().find_position(|c| **c == 2019).unwrap().0
        );
    }
}

fn muladd(state: &HashMap<i128, (i128, i128)>, p: i128, m: i128, reps: i64) -> i128 {
    if reps == 0 {
        return p;
    }

    let mut pow2 = 0;
    while (1 << (pow2 + 1)) < reps {
        pow2 += 1;
    }

    println!("{} = 1<<{} + {}", reps, pow2, reps - (1 << pow2));
    let (pa, pb) = state.get(&pow2).unwrap();
    let q = modulo(p * pa + pb, m);

    muladd(state, q, m, reps - (1 << pow2))
}

fn run3(title: &str, input: &str, count: i64, reps: i64, mut pos: i64) {
    let steps = input_parser::file(input).unwrap();

    println!("{}\n", expand_str(&steps, count, "pos".to_owned()));
    println!("{:?}\n", expand(&steps, count));

    let simp = simplify(expand(&steps, count), count as i128);
    println!("{:#?}\n", simp);

    println!("expand: {}", evaluate(&expand(&steps, count), count as i128, pos as i128) as i64);
    println!("simply: {}", evaluate(&simplify(expand(&steps, count), count as i128), count as i128, pos as i128) as i64);

    // p[n+1] = (p[n] * 14101252936313) + 37661399858180
    // p[n+1] = (p[n] * a) + b
    // p[n+2] = (p[n] * a^2) + b*a + b = (p[n] * c) + d
    // p[n+4] = (p[n] * c^2) + d*c + d ...

    let m = count as i128;
    let mut state = HashMap::new();
    state.insert(0, (14101252936313, 37661399858180));
    for i in 1..64 {
        let (a, b) = state[&(i-1)];
        state.insert(i, (modulo(a * a, m), modulo(b * a + b, m)));
    }

    let pos2 = muladd(&state, pos as i128, m, reps) as i64;
    println!("{} part 2: {}", title, pos2);

    if reps < 1000000 {
        for i in 0..reps {
            // pos = card_at_pos(&steps, count, pos);
            pos = evaluate(&simp, count as i128, pos as i128) as i64;

            // println!("{}", pos);
        }
        println!("{} part 2: {}", title, pos);
    }
}


fn main() {
    run2("demo 1", "deal with increment 7
deal into new stack
deal into new stack
", 10);

    run2("demo 2", "cut 6
deal with increment 7
deal into new stack
", 10);

    run2("demo 3", "deal with increment 7
deal with increment 9
cut -2
", 10);

    run2("demo 4", "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
", 10);

    run2(
        "input",
        &std::fs::read_to_string("22/input.txt").unwrap(),
        10007,
    );

    // run3(
    //     "input",
    //     &std::fs::read_to_string("22/input.txt").unwrap(),
    //     10007,
    //     1,
    //     7171,
    // );

    run3(
        "input",
        &std::fs::read_to_string("22/input.txt").unwrap(),
        119315717514047,
        101741582076661,
        2020
    );
}
