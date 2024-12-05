use std::collections::HashMap;

peg::parser! {
    grammar input_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule order() -> (u32, u32)
            = a:number() "|" b:number() "\n" { (a, b) }

        rule update() -> Vec<u32>
            = a:(number() ++ ",") "\n" { a }

        pub rule file() -> (Vec<(u32, u32)>, Vec<Vec<u32>>)
            = o:order()+ "\n" u:update()+ { (o, u) }
    }
}

fn run(title: &str, input: &str) {
    let (orders, updates) = input_parser::file(input).unwrap();

    println!("{:?}", orders);
    println!("{:?}", updates);

    let mut rel = HashMap::new();
    for o in &orders {
        rel.insert((o.0, o.1), std::cmp::Ordering::Less);
        rel.insert((o.1, o.0), std::cmp::Ordering::Greater);
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for u in &updates {
        let ok = u.windows(2).all(|p| {
            !orders.iter().any(|&(a, b)| a == p[1] && b == p[0])
        });
        // println!("{} {:?}", ok, u);
        if ok {
            part1 += u[u.len() / 2];
        } else {
            let mut u = u.clone();
            u.sort_by(|&a, &b| rel.get(&(a, b)).copied().unwrap());

            // println!("-- {:?}", u);
            part2 += u[u.len() / 2];
        }
    }

    println!("{} part 1: {}", title, part1);
    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
