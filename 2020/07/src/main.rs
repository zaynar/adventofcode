// Part 1: 10 mins
// Part 1+2: 13 mins

use std::collections::HashMap;

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule ident() -> String
            = i:$(['a'..='z']+) { i.to_owned() }

        rule bags() -> String
            = i:$(ident() " " ident()) " " ("bags" / "bag") { i.to_owned() }

        rule line() -> (String, Vec<(i64, String)>)
            = a:bags() " contain " b:(
                b:(n:number() " " c:bags() { (n, c) }) ++ ", " { b }
                / "no other bags" { Vec::new() }
            ) "." { (a, b) }

        pub rule file() -> Vec<(String, Vec<(i64, String)>)>
            = (l:line() "\n" { l })*
    }
}

fn has_gold(k: String, data: &HashMap<String, Vec<(i64, String)>>) -> bool {
    if k == "shiny gold" {
        return true;
    }

    return data[&k].iter().any(|c| has_gold(c.1.clone(), data));
}

fn contains(k: String, data: &HashMap<String, Vec<(i64, String)>>) -> i64 {
    return 1 + data[&k].iter().map(|c| c.0 * contains(c.1.clone(), data)).sum::<i64>();
}

fn run(title: &str, input: &str) {

    let data: HashMap<String, Vec<(i64, String)>> = HashMap::from_iter(input_parser::file(input).unwrap().iter().cloned());
    // println!("{:?}", data);

    let part1 = data.keys().filter(|&k| {
        k != "shiny gold" && has_gold(k.clone(), &data)
    }).count();

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, contains("shiny gold".to_string(), &data) - 1);
}

const INPUT_DEMO: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
