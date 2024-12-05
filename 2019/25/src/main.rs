use std::{collections::{HashMap, VecDeque}, ops::Deref};

use intcode::RunState;
use itertools::Itertools;

mod intcode;

#[derive(Debug, Clone)]
struct Room {
    name: String,
    desc: String,
    dirs: Vec<String>,
    items: Vec<String>,
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i64
            = n:$("-"? ['0'..='9']+) {? n.parse().or(Err("number")) }

        rule dir() -> String
            = s:$("north" / "south" / "east" / "west") { s.to_owned() }

        rule dirs() -> Vec<String>
            = ("- " d:dir() "\n" { d })+

        rule item() -> String
            = s:$([^'\n']+) { s.to_owned() }

        rule items() -> Vec<String>
            = ("- " i:item() "\n" { i })+

        rule room() -> Room
            = "\n"* "== " name:$(([^'=' | ' ']+) ++ " ") " ==\n"
              desc:$([^'\n']+)
              "\n\nDoors here lead:\n" d:dirs()
              i:("\nItems here:\n" i:items() { i } / { Vec::new() })
              "\nCommand?\n" {
                Room { name: name.to_owned(), desc: desc.to_owned(), dirs: d, items: i }
            }

        pub rule output() -> Option<Room>
            = r:room() { Some(r) }
            / "\nYou take the " [_]* { None }
            / "\nYou drop the " [_]* { None }

    }
}

fn probe(state: &intcode::State, take: &Vec<&str>, dir_inputs: Vec<String>) -> Option<intcode::State> {
    let mut state = state.clone();

    let mut inputs: VecDeque<String> = VecDeque::from(dir_inputs);
    let mut in_char = 0;

    let mut output = Vec::new();

    let mut rooms = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    'OUTER: loop {
        match state.runstate {
            RunState::Halted => {
                println!("[Halted]");
                if !output.is_empty() {
                    let text =  output.iter().join("");
                    println!("{:?}", text);
                }
                return None;
            }
            RunState::Ready => state.step(),
            RunState::HasInput(_) => state.step(),
            RunState::NeedsInput => {
                if !output.is_empty() {
                    let text =  output.iter().join("");

                    if text.contains("ejected back to the checkpoint") {
                        return None;
                    }

                    // println!("{:?}", text);
                    let parsed = input_parser::output(text.as_str());
                    if let Ok(Some(room)) = parsed {
                        println!("{:?}", room);
                        rooms.insert((x, y), room.clone());

                        for item in &room.items {
                            if take.contains(&item.as_str()) {
                                inputs.push_front(format!("take {}", item));
                            }
                        }
                    } else {
                        println!("{}", text);
                        println!("{:?}", parsed);
                        parsed.expect("parser");
                    }
                    output.clear();
                }

                if inputs.is_empty() {
                    println!("[End of input]");
                    break 'OUTER;
                }

                let c;
                if in_char == 0 {
                    print!("<");

                    match inputs[0].as_str() {
                        "north" => y -= 1,
                        "south" => y += 1,
                        "west" => x -= 1,
                        "east" => x += 1,
                        _ => (),
                    }
                }
                if in_char == inputs[0].len() {
                    c = '\n';
                    print!(">");
                    inputs.pop_front();
                    in_char = 0;
                } else {
                    c = inputs[0].as_bytes()[in_char] as char;
                    in_char += 1;
                    print!("{}", c);
                }
                state.runstate = RunState::HasInput(c as i64);
            },
            RunState::HasOutput(n) => {
                if n < 128 {
                    // print!("{}", n as u8 as char);
                    output.push(n as u8 as char);
                } else {
                    println!("[{}]", n);
                }
                state.runstate = RunState::Ready;
            }
        }
    }

    let items = rooms.values().map(|r| &r.items).flatten().sorted().dedup().collect_vec();
    println!("Items: {:?}", items);

    println!();
    for (coords, room) in rooms.clone() {
        println!("{:?} {:?}", coords, room);

        for dir in &room.dirs {
            let nc = match dir.as_str() {
                "north" => (coords.0, coords.1 - 1),
                "south" => (coords.0, coords.1 + 1),
                "west" => (coords.0 - 1, coords.1),
                "east" => (coords.0 + 1, coords.1),
                _ => panic!(),
            };
            rooms.entry(nc).or_insert_with(|| Room { name: "?".to_owned(), desc: String::new(), dirs: Vec::new(), items: Vec::new() });
        }
    }

    for y in -2..=5 {
        for x in -3..=5 {
            print!("{:25}| ", rooms.get(&(x, y)).map_or("", |r| &r.name));
        }
        println!();
    }

    Some(state)
}

fn run(title: &str, input: &str) {
    let initial = intcode::load(input);

    let items = vec![
        "astronaut ice cream",
        "bowl of rice",
        "easter egg",
        //"escape pod",
        // "giant electromagnet",
        //"infinite loop",
        // "molten lava",
        "mutex",
        "ornament",
        //"photons",
        "tambourine",
        "whirled peas"
    ];

    let checkpoint = probe(&initial, &items, vec![
        // Hull Breach
        "east","west",
        "west", // Navigation
        // "take whirled peas",
        "east", // Hull Breach
        "south", // Gift Wrapping Centre
        // "take photons",
        "west","east",
        "east", // Holodeck
        // "take mutex",
        "east", // Hot Chocolate Fountain
        // "take astronaut ice cream"
        "east", // Stables
        // "take ornament"
        "east", // Corridor
        "west", // Stables
        "west", // Hot Chocolate Fountain
        "south", // Engineering
        // "take tambourine"
        "north", // Hot Chocolate Fountain
        "west", // Holodeck
        "south", // Passages
        // "take escape pod"
        "south", // Science Lab
        "east", // Observatory
        "west",
        "west",
        "south",
        "west",
        // "west",
    ].iter().map(|&s| s.to_owned()).collect()).unwrap();

    for drop in items.iter().powerset() {
        println!("{:?}", drop);
        let mut inputs = drop.iter().map(|&s| format!("drop {}", s) ).collect_vec();
        inputs.push("west".to_owned());

        if let Some(state) = probe(&checkpoint, &items, inputs) {
            break;
        }

    }

}

fn main() {
    run("input", &std::fs::read_to_string("25/input.txt").unwrap());
}
