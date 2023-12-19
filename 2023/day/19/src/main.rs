use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Rule {
    Cmp { field: String, gt: bool, val: u32, next: String },
    Always(String),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct InputFile {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<HashMap<String, u32>>,
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule ident() -> String
            = i:$(['a'..='z' | 'A'..='Z']+) { i.to_string() }

        rule rule_cmp() -> Rule
            = field:ident() op:$("<" / ">") val:number() ":" next:ident() { Rule::Cmp { field, gt: op == ">", val, next }}

        rule rule_always() -> Rule
            = field:ident() { Rule::Always(field) }

        rule rule_() -> Rule
            = rule_cmp() / rule_always()

        rule workflow() -> (String, Vec<Rule>)
            = name:ident() "{" rules:(rule_() ** ",") "}\n" { (name, rules) }

        rule part_field() -> (String, u32)
            = name:ident() "=" val:number() { (name, val) }

        rule part() -> HashMap<String, u32>
            = "{" fs:(part_field() ** ",") "}\n" { HashMap::from_iter(fs) }

        pub rule file() -> InputFile
            = workflows:(workflow()+) "\n" parts:(part()+) { InputFile { workflows: HashMap::from_iter(workflows), parts } }
    }
}

fn process(part: &HashMap<String, u32>, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut state = "in";
    loop {
        for rule in workflows.get(state).expect(state) {
            // println!(" {} {:?}", state, rule);
            let next = match rule {
                Rule::Always(next) => next,
                Rule::Cmp { field, gt, val, next } => if (*gt && part[field] > *val) || (!*gt && part[field] < *val) { next } else { continue; }
            };
            // println!(" -> {}", next);
            if next == "A" {
                return true;
            }
            if next == "R" {
                return false;
            }
            state = next;
            break;
        }
    }
}

#[derive(Debug)]
struct PartGroup<'a> {
    state: String,
    rule: usize,
    min: HashMap<&'a str, u32>,
    max: HashMap<&'a str, u32>,
}

fn main() {
    let input = input_parser::file(&fs::read_to_string("input").unwrap()).unwrap();

    // println!("{:?}", input);

    let mut answer = 0;
    for part in input.parts {
        let accept = process(&part, &input.workflows);
        // println!("{} {:?}", accept, part);
        if accept {
            answer += part.values().sum::<u32>();
        }
    }

    println!("Part 1: {:?}", answer);

    let mut pgs = Vec::new();
    pgs.push(PartGroup { state: "in".to_string(), rule: 0, min: HashMap::from(
        [("x", 1), ("m", 1), ("a", 1), ("s", 1)]
    ), max: HashMap::from(
        [("x", 4000), ("m", 4000), ("a", 4000), ("s", 4000)]
    )});

    let mut accepted = 0;
    while !pgs.is_empty() {
        // println!("{:?}", pgs);

        let mut new_pgs = Vec::new();

        for pg in pgs {
            if pg.state == "A" {
                accepted += ["x", "m", "a", "s"].iter().map(|c|
                    (pg.max[c] - pg.min[c] + 1) as u64
                ).product::<u64>();
                continue;
            } else if pg.state == "R" {
                continue;
            }

            let rule = &input.workflows.get(pg.state.as_str()).unwrap()[pg.rule];
            // println!(" {} {:?}", state, rule);
            match rule {
                Rule::Always(next) => {
                    new_pgs.push(PartGroup { state: next.to_string(), rule: 0, min: pg.min, max: pg.max });
                },
                Rule::Cmp { field, gt, val, next } => {
                    let field = field.as_str();
                    let val = *val;
                    if (*gt && pg.min[field] > val) || (!*gt && pg.max[field] < val) {
                        // All matched
                        // println!("all");
                        new_pgs.push(PartGroup { state: next.clone(), rule: 0, min: pg.min, max: pg.max });
                    } else if (*gt && pg.max[field] <= val) || (!*gt && pg.min[field] >= val) {
                        // None matched
                        // println!("none");
                        new_pgs.push(PartGroup { state: pg.state, rule: pg.rule + 1, min: pg.min, max: pg.max });
                    } else {
                        // Split
                        // println!("split");
                        let mut min_hit = pg.min.clone();
                        let mut min_miss = pg.min.clone();
                        let mut max_hit = pg.max.clone();
                        let mut max_miss = pg.max.clone();
                        if *gt {
                            min_hit.insert(field, val + 1);
                            max_miss.insert(field, val);
                        } else {
                            max_hit.insert(field, val - 1);
                            min_miss.insert(field, val);
                        }
                        new_pgs.push(PartGroup { state: next.clone(), rule: 0, min: min_hit, max: max_hit });
                        new_pgs.push(PartGroup { state: pg.state, rule: pg.rule + 1, min: min_miss, max: max_miss });
                    }
                }
            };
        }

        pgs = new_pgs;
    }

    println!("Part 2: {:?}", accepted);
}
