use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Wire {
    Set(u16, String),
    And(String, String, String),
    Or(String, String, String),
    Copy(String, String),
    And1(String, String),
    Not(String, String),
    Lshift(String, u16, String),
    Rshift(String, u16, String),
}

peg::parser! {
    grammar input_parser() for str {
        rule num() -> u16
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule ident() -> String
            = s:$(['a'..='z']+) { s.to_owned() }

        rule copy() -> Wire = a:ident() " -> " r:ident() "\n" { Wire::Copy(a, r) }
        rule and1() -> Wire = "1 AND " a:ident() " -> " r:ident() "\n" { Wire::And1(a, r) }

        rule set() -> Wire = n:num() " -> " r:ident() "\n" { Wire::Set(n, r) }
        rule and() -> Wire = a:ident() " AND " b:ident() " -> " r:ident() "\n" { Wire::And(a, b, r) }
        rule or() -> Wire = a:ident() " OR " b:ident() " -> " r:ident() "\n" { Wire::Or(a, b, r) }
        rule not() -> Wire = "NOT " a:ident() " -> " r:ident() "\n" { Wire::Not(a, r) }
        rule lshift() -> Wire = a:ident() " LSHIFT " n:num() " -> " r:ident() "\n" { Wire::Lshift(a, n, r) }
        rule rshift() -> Wire = a:ident() " RSHIFT " n:num() " -> " r:ident() "\n" { Wire::Rshift(a, n, r) }

        pub rule file() -> Vec<Wire>
            = (copy() / and1() / set() / and() / or() / not() / lshift() / rshift())+
    }
}

fn run(title: &str, input: &str) {
    let mut data = input_parser::file(input).unwrap();

    println!("{:?}", data);

    let mut signals: HashMap<String, u16> = HashMap::new();

    loop {
        let old = signals.clone();

        for w in &data {
            match w.clone() {
                Wire::Set(n, r) => { signals.insert(r, n); },
                Wire::And(a, b, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) & signals.get(&b).copied().unwrap_or(0)); },
                Wire::Or(a, b, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) | signals.get(&b).copied().unwrap_or(0)); },
                Wire::Copy(a, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0)); },
                Wire::And1(a, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) & 1); },
                Wire::Not(a, r) => { signals.insert(r, !signals.get(&a).copied().unwrap_or(0)); },
                Wire::Lshift(a, n, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) << n); },
                Wire::Rshift(a, n, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) >> n); },
            }
        }

        if old == signals {
            break;
        }
    }

    // println!("{:?}", signals);

    println!("{} part 1: {:?}", title, signals["a"]);

    let old_a = signals["a"];
    data = data.iter().map(|w| match w {
        Wire::Set(_, r) if r == "b" => {
            Wire::Set(old_a, r.clone())
        }
        w => w.clone()
    }).collect();
    signals.clear();

    loop {
        let old = signals.clone();

        for w in &data {
            match w.clone() {
                Wire::Set(n, r) => { signals.insert(r, n); },
                Wire::And(a, b, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) & signals.get(&b).copied().unwrap_or(0)); },
                Wire::Or(a, b, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) | signals.get(&b).copied().unwrap_or(0)); },
                Wire::Copy(a, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0)); },
                Wire::And1(a, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) & 1); },
                Wire::Not(a, r) => { signals.insert(r, !signals.get(&a).copied().unwrap_or(0)); },
                Wire::Lshift(a, n, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) << n); },
                Wire::Rshift(a, n, r) => { signals.insert(r, signals.get(&a).copied().unwrap_or(0) >> n); },
            }
        }

        if old == signals {
            break;
        }
    }

    println!("{} part 2: {:?}", title, signals["a"]);
}

const INPUT_DEMO: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
