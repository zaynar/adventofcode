use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum E {
    hydrogen,
    lithium,
    polonium,
    thulium,
    promethium,
    ruthenium,
    cobalt,
    elerium,
    dilithium,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Item {
    Generator(E),
    Chip(E),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    steps: u32,
    elevator: u32,
    items: Vec<(Item, u32)>,
}


// Cannot leave chip with incompatible generator, unless it has its own generator
// (Can leave generator on its own)
// Can move two items per step; must contain at least one

fn valid(state: &State) -> bool {
    // println!("### {:?}", state);
    for floor in 1..=4 {
        let has_gen = state.items.iter().any(|(k, v)| match k {
            Item::Generator(_) => *v == floor,
            Item::Chip(_) => false,
        });
        let has_unmatched = state.items.iter().any(|(k, v)| match k {
            Item::Generator(_) => false,
            Item::Chip(n) => *v == floor && state.items.iter().find(|(k, v)| *k == Item::Generator(*n)).unwrap().1 != floor,
        });

        // println!("- {} {} {}", floor, has_gen, has_unmatched);
        if has_gen && has_unmatched {
            return false;
        }
    }

    true
}

fn moves(state: &State) -> Vec<State> {
    // println!("@@ {:?}", state);

    let here = state.items.iter().enumerate().filter(|(i, (it, f))| *f == state.elevator);
    let mut moves = here.clone().combinations(1).chain(here.combinations(2)).collect_vec();

    let mut ret = Vec::new();

    for floor in [state.elevator - 1, state.elevator + 1] {
        if floor < 1 || floor > 4 {
            continue;
        }

        for m in moves.iter_mut() {
            let mut new_state = state.clone();
            new_state.elevator = floor;
            new_state.steps = state.steps + 1;

            for i in m {
                new_state.items[i.0].1 = floor;
            }

            // println!("?? {} {:?}", valid(&new_state), new_state);
            if valid(&new_state) {
                ret.push(new_state);
            }
        }

    }

    ret
}

fn run(title: &str, state: State) {

    // println!("{:?}", state);

    let mut open = VecDeque::new();
    let mut closed = HashSet::new();
    open.push_front(state.clone());
    closed.insert((state.elevator, state.items.clone()));

    let mut i = 0;

    while let Some(node) = open.pop_front() {

        i += 1;
        if i % 100_000 == 0{
            println!("{} {} = {:?}", open.len(), closed.len(), node);
        }

        if node.items.iter().all(|(k, v)| *v == 4) {
            println!("{} part 1: {:?}", title, node);
            return;
        }

        for next in moves(&node) {
            // println!("-> {:?}", next);

            if closed.insert((next.elevator, next.items.clone())) {
                open.push_back(next);
            }
        }
    }


    println!("{} part 2: {}", title, "TODO");
}

fn main() {
    let demo = State {
        steps: 0,
        elevator: 1,
        items: vec![
            (Item::Generator(E::hydrogen), 2),
            (Item::Generator(E::lithium), 3),
            (Item::Chip(E::hydrogen), 1),
            (Item::Chip(E::lithium), 1),
        ],
    };

    run("demo", demo);

/*
The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip, a promethium generator, a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator, and a cobalt-compatible microchip.
The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
The third floor contains nothing relevant.
The fourth floor contains nothing relevant.
 */

    let input = State {
        steps: 0,
        elevator: 1,
        items: vec![
            (Item::Generator(E::polonium), 1),
            (Item::Generator(E::thulium), 1),
            (Item::Generator(E::promethium), 1),
            (Item::Generator(E::ruthenium), 1),
            (Item::Generator(E::cobalt), 1),

            (Item::Chip(E::thulium), 1),
            (Item::Chip(E::ruthenium), 1),
            (Item::Chip(E::cobalt), 1),
            (Item::Chip(E::polonium), 2),
            (Item::Chip(E::promethium), 2),
        ],
    };

    run("input", input);

    let input = State {
        steps: 0,
        elevator: 1,
        items: vec![
            (Item::Generator(E::polonium), 1),
            (Item::Generator(E::thulium), 1),
            (Item::Generator(E::promethium), 1),
            (Item::Generator(E::ruthenium), 1),
            (Item::Generator(E::cobalt), 1),

            (Item::Chip(E::thulium), 1),
            (Item::Chip(E::ruthenium), 1),
            (Item::Chip(E::cobalt), 1),
            (Item::Chip(E::polonium), 2),
            (Item::Chip(E::promethium), 2),

            (Item::Generator(E::elerium), 1),
            (Item::Generator(E::dilithium), 1),
            (Item::Chip(E::elerium), 1),
            (Item::Chip(E::dilithium), 1),
        ],
    };

    run("input", input);
}
