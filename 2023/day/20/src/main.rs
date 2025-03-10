use std::{fs, collections::{HashMap, VecDeque}};

#[derive(Debug, Clone)]
enum ModuleType<'a> {
    Broadcast,
    Flipflop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

#[derive(Debug, Clone)]
struct Module<'a> {
    name: &'a str,
    ty: ModuleType<'a>,
    dests: Vec<&'a str>,
}

peg::parser! {
    grammar input_parser() for str {
        rule ident() -> &'input str
            = i:$(['a'..='z' | 'A'..='Z']+) { i }

        rule dests() -> Vec<&'input str>
            = ds:(ident() ** ", ") { ds }

        rule mod_broadcast() -> Module<'input>
            = "broadcaster -> " dests:dests() "\n" { Module { name: "broadcaster", ty: ModuleType::Broadcast, dests } }

        rule mod_ff() -> Module<'input>
            = "%" name:ident() " -> " dests:dests() "\n" { Module { name, ty: ModuleType::Flipflop(false), dests } }

        rule mod_conj() -> Module<'input>
            = "&" name:ident() " -> " dests:dests() "\n" { Module { name, ty: ModuleType::Conjunction(HashMap::new()), dests } }

        rule mod() -> Module<'input>
            = mod_broadcast() / mod_ff() / mod_conj()

        pub rule file() -> Vec<Module<'input>>
            = modules:(mod()+) { modules }
    }
}

fn init<'a>(modules: &mut HashMap<&'a str, Module<'a>>) {
    for m in modules.clone().values() {
        for d in &m.dests {
            if let Some(dm) = modules.get_mut(d) {
                if let ModuleType::Conjunction(ins) = &mut dm.ty {
                    ins.insert(m.name, false);
                }
            } else {
                println!("Undefined {}", d);
            }
        }
    }
}

fn press(modules: &mut HashMap<&str, Module<'_>>, low: &mut u32, high: &mut u32, i: u32) -> bool {
    let mut pulses = VecDeque::new();
    pulses.push_back(("button", "broadcaster", false));

    while let Some((src, dst, pulse)) = pulses.pop_front() {
        // println!("{} -{}-> {}", src, pulse, dst);

        if pulse {
            *high += 1;
        } else {
            *low += 1;
        }

        if pulse == false && dst == "rx" {
            return true;
        }

        // Relevant nodes determined by looking at Graphviz
        if (dst == "gs" || dst == "vg" || dst == "kd" || dst == "zf") && pulse == false {
            println!("{} {}", dst, i + 1);
            // This finds the period of each binary-counter subgraph.
            // To solve the puzzle, manually calculate the LCM of each period
        }

        if let Some(m) = modules.get_mut(dst) {
            match &mut m.ty {
                ModuleType::Broadcast => {
                    pulses.extend(m.dests.iter().map(|&d| (m.name, d, pulse)));
                },
                ModuleType::Flipflop(st) => {
                    if pulse == false {
                        *st = !*st;
                        pulses.extend(m.dests.iter().map(|&d| (m.name, d, *st)));
                    }
                },
                ModuleType::Conjunction(st) => {
                    *st.get_mut(src).unwrap() = pulse;
                    pulses.extend(m.dests.iter().map(|&d| (m.name, d, !st.values().all(|&i| i))));
                },
            }
        }
    }

    return false;
}

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let input = input_parser::file(&file).unwrap();

    let mut modules = HashMap::from_iter(input.iter().map(|m| (m.name, m.clone())));
    init(&mut modules);

    println!("{:?}", modules);

    let mut low = 0;
    let mut high = 0;
    // for i in 0..1000 {
    //     press(&mut modules, &mut low, &mut high);
    // }
    // println!("Answer 1: {} * {} = {}", low, high, low * high);

    for i in 0..1000000000 {
        if press(&mut modules, &mut low, &mut high, i) {
            println!("Answer 2: {}", i);
            break;
        }

        if i % 100000 == 0 {
            println!("{}...", i);
        }
    }
}
