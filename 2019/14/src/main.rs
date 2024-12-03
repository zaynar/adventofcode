use std::{
    cmp::{self, Ordering}, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Chemical {
    name: String,
    qty: i64,
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule ident() -> String
            = i:$(['A'..='Z']+) { i.to_owned() }

        rule chemical() -> Chemical
            = qty:number() " " name:ident() { Chemical { name, qty } }

        rule line() -> (Vec<Chemical>, Chemical)
            = inputs:(chemical() ++ ", ") " => " output:chemical() { (inputs, output) }

        pub rule file() -> Vec<(Vec<Chemical>, Chemical)>
            = (l:line() "\n" { l })*
    }
}

fn run(title: &str, input: &str) {
    let data = input_parser::file(input).unwrap();
    let reactions = data.iter().fold(HashMap::new(), |mut hash, (inputs, output)| {
        hash.insert(output.name.clone(), (output.qty, inputs));
        hash
    });

    // println!("{:?}", reactions);

    let mut amounts: HashMap<String, i64> = HashMap::new();
    amounts.insert("FUEL".to_owned(), -1);

    while let Some((name, amount)) = amounts.iter().filter(|&(name, qty)| name != "ORE" && *qty < 0).next() {
        let (qty, inputs) = reactions.get(name).unwrap();
        let reps = (-amount + qty - 1) / qty;

        // println!("need {} {} : {}", name, amount, reps);

        *amounts.entry(name.clone()).or_insert(0) += qty * reps;
        for input in inputs.iter() {
            *amounts.entry(input.name.clone()).or_insert(0) -= input.qty * reps;
        }

        // println!("{:?}", amounts);

        // break;
    }

    println!("{} part 1: {}", title, -amounts["ORE"]);
}

fn run2(title: &str, input: &str) {
    let data = input_parser::file(input).unwrap();
    let reactions = data.iter().fold(HashMap::new(), |mut hash, (inputs, output)| {
        hash.insert(output.name.clone(), (output.qty, inputs));
        hash
    });

    // println!("{:?}", reactions);

    let mut low = 0;
    let mut high = 1000000000;

    let mut max = 0;

    'OUTER: while low <= high {
        let mid = (low + high) / 2;

        let mut amounts: HashMap<String, i64> = HashMap::new();
        amounts.insert("FUEL".to_owned(), -mid);
        while let Some((name, amount)) = amounts.iter().filter(|&(name, qty)| name != "ORE" && *qty < 0).next() {
            let (qty, inputs) = reactions.get(name).unwrap();
            let reps = (-amount + qty - 1) / qty;
            *amounts.entry(name.clone()).or_insert(0) += qty * reps;
            for input in inputs.iter() {
                *amounts.entry(input.name.clone()).or_insert(0) -= input.qty * reps;
            }
        }

        let ore = -amounts["ORE"];

        // println!("{} part 2: {} -> {}", title, mid, ore);

        match ore.cmp(&1000000000000) {
            Ordering::Equal => { max = mid; break 'OUTER }
            Ordering::Less => { max = mid; low = mid + 1 }
            Ordering::Greater => { high = mid - 1 }
        }
    }

    println!("{} part 2: {}", title, max);
}


const INPUT_DEMO: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("14/input.txt").unwrap());

    run2("demo", "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
");
    run2("input", &std::fs::read_to_string("14/input.txt").unwrap());
}
