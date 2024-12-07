use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Target {
    Bot(u32),
    Output(u32),
    Input(u32),
}

#[derive(Debug)]
enum Rule {
    Initial(u32, u32),
    Share(u32, Target, Target),
}

peg::parser! {
    grammar input_parser() for str {
        rule num() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule target() -> Target
            = "bot " n:num() { Target::Bot(n) }
            / "input " n:num() { Target::Input(n) }
            / "output " n:num() { Target::Output(n) }

        rule initial() -> Rule
            = "value " a:num() " goes to bot " b:num() "\n" { Rule::Initial(a, b) }

        rule share() -> Rule
            = "bot " a:num() " gives low to " b:target() " and high to " c:target() "\n" { Rule::Share(a, b, c) }

        pub rule file() -> Vec<Rule>
            = (initial() / share())+
    }
}

fn run(title: &str, input: &str) {

    let data = input_parser::file(input).unwrap();

    // println!("{:?}", data);

    let mut state: HashMap<Target, Vec<u32>> = HashMap::new();

    for rule in &data {
        if let Rule::Initial(a, b) = rule {
            state.entry(Target::Bot(*b)).or_insert_with(|| Vec::new()).push(*a);
        }
    }

    println!("{:?}", state);

    loop {
        let mut idle = true;
        for rule in &data {
            if let Rule::Share(a, b, c) = rule {
                if let Some(chips) = state.get(&Target::Bot(*a)) {
                    let chips = chips.clone();
                    if chips.len() == 2 {
                        let min = chips.iter().min().unwrap();
                        let max = chips.iter().max().unwrap();

                        if *min == 17 && *max == 61 {
                            println!("{} part 1: {}", title, a);
                            // return;
                        }

                        state.entry(*b).or_insert_with(|| Vec::new()).push(*min);
                        state.entry(*c).or_insert_with(|| Vec::new()).push(*max);
                        state.get_mut(&Target::Bot(*a)).unwrap().clear();
                        idle = false;
                    }
                }
            }
        }

        // println!("{:?}", state);

        if idle {
            break;
        }
    }

    // for (k,v) in &state {
    //     if let Target::Output(n) = k {
    //         println!("{:?} {:?}", k, v);
    //     }
    // }

    println!("{} part 2: {}", title, state[&Target::Output(0)][0] * state[&Target::Output(1)][0] * state[&Target::Output(2)][0]);
}

const INPUT_DEMO: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
