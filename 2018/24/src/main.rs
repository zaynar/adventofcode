// Part 1: 53 mins
// Part 1+2: 78 mins

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Group {
    id: String,
    owner: i32,
    count: i32,
    hp: i32,
    immune: Vec<String>,
    weak: Vec<String>,
    dmg: i32,
    ty: String,
    init: i32,
}

peg::parser! {
    grammar input_parser() for str {

        rule num() -> i32
           = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule elements() -> Vec<String>
            = (s:$(['a'..='z']+) { s.to_owned() }) ++ ", "

        rule immweak() -> (Vec<String>, Vec<String>)
            = " (immune to " i:elements() ")" { (i, Vec::new()) }
            / " (weak to " w:elements() ")" { (Vec::new(), w) }
            / " (immune to " i:elements() "; weak to " w:elements() ")" { (i, w) }
            / " (weak to " w:elements() "; immune to " i:elements() ")" { (i, w) }
            / { (Vec::new(), Vec::new()) }

        rule group() -> Group
            = count:num() " units each with " hp:num() " hit points"
                immweak:immweak()
            " with an attack that does " dmg:num() " " ty:$(['a'..='z']+) " damage at initiative " init:num() "\n"
            {
                Group { id: "".to_owned(), owner: -1, count, hp, immune: immweak.0, weak: immweak.1, dmg, ty: ty.to_owned(), init }
            }

        pub rule file() -> (Vec<Group>, Vec<Group>)
            = "Immune System:\n" imm:(group()+) "\nInfection:\n" inf:(group()+) { (imm, inf) }
    }
}

fn damage(atk: &Group, def: &Group) -> i32 {
    let mul = if def.immune.contains(&atk.ty) { 0 } else if def.weak.contains(&atk.ty) { 2 } else { 1 };
    (atk.count * atk.dmg * mul)
}

fn run(title: &str, input: &str) {

    let data = input_parser::file(input).unwrap();

    const VERBOSE: bool = false;

    'OUTER: for boost in 0.. {
    // 'OUTER: for boost in 1570.. {

        let mut groups = Vec::new();
        for (i, g) in data.0.iter().enumerate() {
            let mut g = g.clone();
            g.id = format!("Immune {}", i+1);
            g.owner = 0;
            g.dmg += boost;
            groups.push(Rc::new(RefCell::new(g)));
        }
        for (i, g) in data.1.iter().enumerate() {
            let mut g = g.clone();
            g.id = format!("Infection {}", i+1);
            g.owner = 1;
            groups.push(Rc::new(RefCell::new(g)));
        }

        let mut seen = HashSet::new();

        for i in 0.. {

            groups = groups.iter().filter(|g| g.borrow().count > 0).cloned().collect_vec();

            groups.sort_by_key(|g| { let g = g.borrow(); g.id.clone() });
            if !seen.insert(groups.iter().map(|g| g.borrow().clone()).collect_vec()) {
                println!("Boost {}, deadlock", boost);
                // return;
                continue 'OUTER;
            }

            if VERBOSE {
                println!("=====");
                for g in &groups {
                    println!("{} has {}", g.borrow().id, g.borrow().count);
                }
                println!("Boost {}, Total: {}", boost, groups.iter().map(|g| {
                    let g = g.borrow();
                    g.count.max(0)
                }).sum::<i32>());
            }

            groups.sort_by_key(|g| { let g = g.borrow(); (-g.count * g.dmg, -g.init) });
            // println!("\n{:?}", groups);

            let mut chosen = Vec::new();

            {
                let mut targeted: HashSet<String> = HashSet::new();
                for atk in &groups {
                    let atkb = atk.borrow();
                    if atkb.count <= 0 {
                        continue;
                    }

                    let mut targets = Vec::new();
                    for def in &groups {
                        let defb = def.borrow();
                        if atkb.owner != defb.owner && defb.count > 0 && !targeted.contains(&defb.id) {
                            let dmg = damage(&*atkb, &*defb);
                            if dmg > 0 {
                                targets.push((dmg, def));
                            }
                        }
                    }
                    targets.sort_by_key(|(dmg, def)| { let def = def.borrow(); (-dmg, -def.count * def.dmg, -def.init) });
                    if targets.is_empty() {
                        if VERBOSE {
                            println!("{} no targets", atkb.id);
                        }
                    } else {
                        let tgt = targets.first().unwrap();
                        if VERBOSE {
                            println!("{} targets {} for {} dmg", atkb.id, tgt.1.borrow().id, tgt.0);
                        }
                        targeted.insert(tgt.1.borrow().id.clone());
                        chosen.push((Rc::clone(atk), Rc::clone(tgt.1)));
                    }
                }
            }

            // println!();

            chosen.sort_by_key(|(atk, tgt)| -atk.borrow().init);
            for (atk, tgt) in chosen {
                let atk = atk.borrow();
                let mut tgt = tgt.borrow_mut();
                if atk.count <= 0 || tgt.count <= 0 {
                    continue;
                }
                let dmg = damage(&*atk, &*tgt);
                if VERBOSE {
                    println!("{} attacks {}, killing {}", atk.id, tgt.id, dmg / tgt.hp);
                }
                tgt.count -= dmg / tgt.hp;
            }

            if !groups.iter().any(|g| { let g = g.borrow(); g.owner == 0 && g.count > 0 }) {
                println!("Boost {}, Player 1 wins: {}", boost, groups.iter().map(|g| {
                    let g = g.borrow();
                    if g.owner == 1 { g.count.max(0) } else { 0 }
                }).sum::<i32>());
                break;
            }
            if !groups.iter().any(|g| { let g = g.borrow(); g.owner == 1 && g.count > 0 }) {
                println!("Boost {}, Player 0 wins: {}", boost, groups.iter().map(|g| {
                    let g = g.borrow();
                    if g.owner == 0 { g.count.max(0) } else { 0 }
                }).sum::<i32>());
                return;
            }

        }
    }

    // Target selection:
    // Decreasing effpower (count*dmg), tie-break highest inititaive
    // Calculate damage to each target, pick max, tie-break target effpower, tie-break highest init
    // Each group can only be targeted once

    // Next phase:
    // By decreasing initiative, deal damage
    // Immune => no damage. Weak => *2
    // count -= damage / hp
}

const INPUT_DEMO: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("24/input.txt").unwrap());
}
