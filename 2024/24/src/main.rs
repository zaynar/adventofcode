// Part 1: 9 mins
// Part 1+2: 96 mins

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Wire {
    Sig(String),
    And(Box<Wire>, Box<Wire>),
    Or(Box<Wire>, Box<Wire>),
    Xor(Box<Wire>, Box<Wire>),
}

fn run(title: &str, input: &str) {
    let (signals, gates) = input.split_once("\n\n").unwrap();

    let mut vals = HashMap::new();

    for s in signals.lines() {
        vals.insert(s[0..3].to_owned(), s.chars().nth(5) == Some('1'));
    }
    loop {
        let mut busy = false;
        for g in gates.lines() {
            let (a, op, b, _, c) = g.split_ascii_whitespace().collect_tuple().unwrap();

            if let Some(a) = vals.get(a) {
                if let Some(b) = vals.get(b) {
                    if !vals.contains_key(c) {
                        vals.insert(c.to_owned(), match op {
                            "AND" => *a && *b,
                            "OR" => *a || *b,
                            "XOR" => *a != *b,
                            _ => panic!(),
                        });
                        busy = true;
                    }
                }
            }
        }
        if !busy { break; }
    }

    // println!("{:?}", vals);
    let xs = vals.iter().sorted().filter_map(|(k, v)| if k.starts_with("x") { Some(v) } else { None }).collect_vec();
    let ys = vals.iter().sorted().filter_map(|(k, v)| if k.starts_with("y") { Some(v) } else { None }).collect_vec();
    let zs = vals.iter().sorted().filter_map(|(k, v)| if k.starts_with("z") { Some(v) } else { None }).collect_vec();
    let part1 = zs.iter().enumerate().map(|(i, v)| if **v { 1 << i } else { 0 }).sum::<u64>();

    // 45 + 45 -> 46
    // println!("x {:?}", xs);
    // println!("y {:?}", ys);
    // println!("z {:?}", zs);

    println!("{} part 1: {:?}", title, part1);
}

// zN must be Xor(Xor(aN, bN), ...only depend on earlier thing)

fn is_valid(v: &Wire, bit: u32) -> bool {
    if bit == 0 { return true; }
    let xN = Wire::Sig(format!("x{:02}", bit));
    let yN = Wire::Sig(format!("y{:02}", bit));
    match v {
        Wire::Xor(a, b) => {
            match (**a).clone() {
                Wire::Xor(c, d) => if (*c == xN && *d == yN) || (*c == yN && *d == xN) {
                    return is_valid_carry(b, bit);
                }
                _ => (),
            }
            match (**b).clone() {
                Wire::Xor(c, d) => if (*c == xN && *d == yN) || (*c == yN && *d == xN) {
                    return is_valid_carry(a, bit);
                }
                _ => (),
            }
            return false;
        }
        _ => false,
    }
}

fn is_valid_carry(v: &Wire, bit: u32) -> bool {
    // println!("carry? {:?} {}", v, bit);
    let xN = format!("x{:02}", bit);
    let yN = format!("y{:02}", bit);
    match v {
        Wire::Sig(s) => {
            return u32::from_str_radix(&s[1..], 10).unwrap() < bit;
        }
        Wire::Xor(a, b) => is_valid_carry(a, bit) && is_valid_carry(b, bit),
        Wire::And(a, b) => is_valid_carry(a, bit) && is_valid_carry(b, bit),
        Wire::Or(a, b) => is_valid_carry(a, bit) && is_valid_carry(b, bit),
    }
}

fn simplify(v: &Wire) -> Wire {
    let mut v = v.clone();
    loop {
        let s = simplify2(&v);
        if s == v {
            return s;
        }
        v = s;
    }
}

fn simplify2(v: &Wire) -> Wire {

    // z = a^b^cin = d^cin
    // cout = (a&b) | (c&(a^b))
    //      =   e   |  c&d
    //      =   e   |  f

    match v {
        Wire::Xor(a, b) => {
            let a = simplify(&**a);
            let b = simplify(&**b);
            match (&a, &b) {
                (Wire::Sig(x), Wire::Sig(y)) if x == "x00" && y == "y00" => return Wire::Sig("z00".to_owned()),
                (Wire::Sig(x), Wire::Sig(y)) if
                    ((x.starts_with("x") && y.starts_with("y")) ||
                    (x.starts_with("y") && y.starts_with("x")))
                    && x[1..] == y[1..] => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("d{:02}", n));
                    }
                (Wire::Sig(x), Wire::Sig(y)) if
                    (x.starts_with("c") && *y == format!("d{:02}", u32::from_str_radix(&x[1..], 10).unwrap() + 1)) => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("z{:02}", n + 1));
                    }
                (Wire::Sig(y), Wire::Sig(x)) if
                    (x.starts_with("c") && *y == format!("d{:02}", u32::from_str_radix(&x[1..], 10).unwrap() + 1)) => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("z{:02}", n + 1));
                    }
                (Wire::Sig(x), Wire::Sig(y)) if
                    ((x.starts_with("e") && y.starts_with("f")) ||
                    (x.starts_with("f") && y.starts_with("e")))
                    && x[1..] == y[1..] => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("c{:02}", n));
                    }
                _ => Wire::Xor(Box::new(a), Box::new(b)),
            }
        }
        Wire::And(a, b) => {
            let a = simplify(&**a);
            let b = simplify(&**b);
            match (&a, &b) {
                (Wire::Sig(x), Wire::Sig(y)) if (x == "x00" && y == "y00") || (y == "x00" && x == "y00") => return Wire::Sig("c00".to_owned()),
                (Wire::Sig(x), Wire::Sig(y)) if
                    ((x.starts_with("x") && y.starts_with("y")) ||
                    (x.starts_with("y") && y.starts_with("x")))
                    && x[1..] == y[1..] => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("e{:02}", n));
                    }
                (Wire::Sig(x), Wire::Sig(y)) if
                    (x.starts_with("c") && *y == format!("d{:02}", u32::from_str_radix(&x[1..], 10).unwrap() + 1)) => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("f{:02}", n + 1));
                    }
                (Wire::Sig(y), Wire::Sig(x)) if
                    (x.starts_with("c") && *y == format!("d{:02}", u32::from_str_radix(&x[1..], 10).unwrap() + 1)) => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("f{:02}", n + 1));
                    }
                _ => Wire::And(Box::new(a), Box::new(b)),
            }
        }
        Wire::Or(a, b) => {
            let a = simplify(&**a);
            let b = simplify(&**b);
            match (&a, &b) {
                (Wire::Sig(x), Wire::Sig(y)) if
                    ((x.starts_with("e") && y.starts_with("f")) ||
                    (x.starts_with("f") && y.starts_with("e")))
                    && x[1..] == y[1..] => {
                        let n = u32::from_str_radix(&x[1..], 10).unwrap();
                        return Wire::Sig(format!("c{:02}", n));
                    }
                _ => Wire::Or(Box::new(a), Box::new(b)),
            }
        }
        Wire::Sig(a) => Wire::Sig(a.clone()),

    }
}

fn run2(title: &str, input: &str) {
    let (signals, gates) = input.split_once("\n\n").unwrap();

    let mut vals = HashMap::new();

    for s in signals.lines() {
        vals.insert(s[0..3].to_owned(), Wire::Sig(s[0..3].to_owned()));
    }

    let mut outs = Vec::new();
    for g in gates.lines() {
        let (a, op, b, _, c) = g.split_ascii_whitespace().collect_tuple().unwrap();
        outs.push(c);
        // println!("{} -> {}; {} -> {};", a, c, b, c);
    }

    // for (swap0, swap1, swap2, swap3) in outs.iter().tuple_combinations()
    // for (swap0, swap1) in outs.iter().tuple_combinations()
    {

        loop {
            let mut busy = false;
            for g in gates.lines() {
                let (a, op, b, _, c) = g.split_ascii_whitespace().collect_tuple().unwrap();

                let c = if c == "fhc" { "z06" }
                else if c == "z06" { "fhc"}
                else if c == "qhj" { "z11"}
                else if c == "z11" { "qhj"}
                else if c == "ggt" { "mwh"}
                else if c == "mwh" { "ggt"}
                else if c == "hqk" { "z35"}
                else if c == "z35" { "hqk"}
                // else if c == *swap0 { swap1 }
                // else if c == *swap1 { swap0 }
                else { c };

                // else if c == *swap2 { swap3 }
                // else if c == *swap3 { swap2 }
                // else { c };

                if let Some(a) = vals.get(a) {
                    if let Some(b) = vals.get(b) {
                        if !vals.contains_key(c) {
                            vals.insert(c.to_owned(), match op {
                                "AND" => Wire::And(Box::new(a.clone()), Box::new(b.clone())),
                                "OR" => Wire::Or(Box::new(a.clone()), Box::new(b.clone())),
                                "XOR" => Wire::Xor(Box::new(a.clone()), Box::new(b.clone())),
                                _ => panic!(),
                            });

                            busy = true;
                        }
                    }
                }
            }
            if !busy { break; }
        }


        // if let Some(w) = vals.get("z23") {
        //     if simplify(&w) == Wire::Sig("z23".to_owned()) {
        //         // println!("{} {}", swap0, swap1);
        //         // break;
        //     }
        // }

        // swap e23, d23

        for i in 0..46 {
            let w = vals.get(&format!("z{:02}", i)).unwrap();
            println!("{} {:?}", i, simplify(&w));
        }

        for k in &outs {
            if simplify(&vals.get(*k).unwrap()) == Wire::Sig("e35".to_owned()) {
                println!("{}", k);
            }
        }

        // let mut valid = 0;
        // // for i in 0..5 {
        // for i in 0..46 {
        //     let w = vals.get(&format!("z{:02}", i)).unwrap();
        //     let v = is_valid(&w, i);
        //     // println!("z{:02} {:?} {}", i, w, v);
        //     // println!("z{:02} {}", i, v);
        //     if v {
        //         valid = i;
        //     } else {
        //         break;
        //     }
        // }

        // println!("swap {:?} {:?} {}", swap0, swap1, valid);
        // if valid > 3 {
        //     println!("swap {} {} {} {}", swap0, swap1, swap2, swap3);
        //     break;
        // }

        // println!("{} part 2: {}", title, "TODO");
    }
}

const INPUT_DEMO: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

fn main() {
    // run("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("24/input.txt").unwrap());
}
