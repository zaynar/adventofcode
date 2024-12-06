use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn reduce(revmap: &HashMap<&str, &str>, chem: String, steps: u32, path: Vec<(&str, &str)>) {
    for i in 0..chem.len().min(20) {
        for (src,dst) in revmap.iter().sorted_by_key(|(src, dst)| -(src.len() as isize)) {
            if chem[i..].starts_with(src) {
                let new = chem[..i].to_owned() + dst + &chem[i+src.len()..];
                if new == "e" {
                    println!("part 2: {}", steps + 1);
                    panic!();
                }

                let mut p = path.clone();
                p.push((src, dst));
                reduce(revmap, new, steps+1, p);
            }
        }
    }
}

fn run(title: &str, input: &str) {
    let mut map = HashMap::new();
    let mut revmap = HashMap::new();

    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (src, dst) = line.split_once(" => ").unwrap();
        map.entry(src).or_insert_with(|| Vec::new()).push(dst);
        assert!(revmap.insert(dst, src).is_none());
        // revmap.entry(dst).or_insert_with(|| Vec::new()).push(src);
    }
    let chem = lines.next().unwrap();

    println!("{:?}", map);
    println!("{:?}", chem);

    let mut outputs = HashSet::new();

    for i in 0..chem.len() {
        if let Some(ds) = map.get(&chem[i..i+1]) {
            for d in ds {
                outputs.insert(format!("{}{}{}", &chem[..i], d, &chem[i+1..]));
            }
        }
        if i+1 < chem.len() {
            if let Some(ds) = map.get(&chem[i..i+2]) {
                for d in ds {
                    outputs.insert(format!("{}{}{}", &chem[..i], d, &chem[i+2..]));
                }
            }
         }
    }

    println!("{} part 1: {}", title, outputs.len());

    reduce(&revmap, chem.to_owned(), 0, Vec::new());

    println!("{} part 2: {}", title, "TODO");
}

fn main() {
//     run("demo", "e => H
// e => O
// H => HO
// H => OH
// O => HH

// HOHOHO
// ");
    run("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
