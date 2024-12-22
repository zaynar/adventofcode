// Part 1: 17 mins
// Part 1+2: 24 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .map(|line| {
            let (i, a) = line.strip_suffix(")").unwrap().split_once(" (contains ").unwrap();
            (i.split_ascii_whitespace().map(|s| s.to_owned()).collect(), a.split(", ").map(|s| s.to_owned()).collect())
        })
        .collect();

    // println!("{:?}", data);

    let ingredients: HashSet<String> = HashSet::from_iter(data.iter().map(|(i, a)| i.clone()).concat());
    let allergens: HashSet<String> = HashSet::from_iter(data.iter().map(|(i, a)| a.clone()).concat());

    let mut link: HashMap<String, HashSet<String>> = HashMap::from_iter(data.iter().map(|(i, a)| i.iter().map(|i| (i.clone(), allergens.clone())).collect_vec()).concat());

    // println!("{:?}", allergens);
    for food in &data {
        for i in &ingredients {
            if !food.0.contains(i) {
                for a in &food.1 {
                    link.get_mut(i).unwrap().remove(a);
                }
            }
        }
    }

    // println!("{:?}", link);

    let safe = link.iter().filter_map(|(k, v)| if v.len() == 0 { Some(k) } else { None }).collect_vec();
    let part1 = data.iter().map(|f|
        f.0.iter().filter(|i| safe.contains(i)).count()
    ).sum::<usize>();

    println!("{} part 1: {}", title, part1);

    let mut assigned = HashMap::new();
    for s in &safe {
        assigned.insert((*s).clone(), "".to_owned());
    }
    loop {
        let mut ch = false;

        for (k, v) in link.iter() {
            let a = v.iter().filter(|i| !assigned.contains_key(*i)).collect_vec();
            if a.len() == 1 {
                assigned.insert(a[0].clone(), k.clone());
                ch = true;
            }
        }

        if !ch { break; }

    }

    println!("{} part 2: {}", title, assigned.iter().filter_map(|(k, v)| if v.is_empty() { None } else { Some((k, v)) }).sorted().map(|(k, v)| v).join(","));
}

const INPUT_DEMO: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("21/input.txt").unwrap());
}
