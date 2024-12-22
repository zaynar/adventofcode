// Part 1: 7 mins
// Part 1+2: 8 mins

use std::collections::HashMap;

fn run(title: &str, input: Vec<usize>) {

    let mut spoken = HashMap::new();

    let mut last = 0;
    // for turn in 0..2020 {
    for turn in 0..30000000 {
        if turn < input.len() {
            last = input[turn];
            spoken.insert(last, (turn, None));
        } else {
            if let Some((prev, Some(prev2))) = spoken.get(&last) {
                last = prev - prev2;
            } else {
                last = 0;
            }
            if let Some((p, _)) = spoken.get(&last) {
                spoken.insert(last, (turn, Some(*p)));
            } else {
                spoken.insert(last, (turn, None));
            }
        }
        // println!("{}", last);
    }

    println!("{} part N: {}", title, last);
}

fn main() {
    run("demo", vec![0,3,6]);
    run("input", vec![15,12,0,14,3,1]);
}
