use std::{fs, collections::HashMap};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "game.pest"]
pub struct GameParser;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let parse = GameParser::parse(Rule::file, &file).unwrap().next().unwrap();

    let mut answer = 0;
    let mut answer2 = 0;

    for record in parse.into_inner() {
        match record.as_rule() {
            Rule::record => {
                let mut inner = record.into_inner();
                let id: u32 = inner.next().unwrap().as_str().parse().unwrap();
                let sets = inner.next().unwrap();

                println!("Game {:?}", id);

                let mut possible = true;
                let mut maxes = HashMap::new();

                for set in sets.into_inner() {
                    println!("  {:?}", set.as_str());
                    for cubes in set.into_inner() {
                        let mut inner = cubes.into_inner();
                        let count: u32 = inner.next().unwrap().as_str().parse().unwrap();
                        let colour = inner.next().unwrap().as_str();
                        println!("    {:?} {:?}", count, colour);

                        let limit = match colour {
                            "red" => 12,
                            "green" => 13,
                            "blue" => 14,
                            _ => unreachable!(),
                        };

                        if count > limit {
                            possible = false;
                        }

                        maxes.insert(colour, u32::max(count, maxes.get(colour).copied().unwrap_or(0)));
                    }
                }

                let mut power = 1;
                for max in maxes.values() {
                    power *= max;
                }
                answer2 += power;

                if possible {
                    answer += id;
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("Answer: {}", answer);
    println!("Answer: {}", answer2);
}
