// Part 1: 6 mins
// Part 1+2: 13 mins

use std::collections::HashMap;

use itertools::Itertools;

fn expand(data: &Vec<u64>, reps: usize, cache: &mut HashMap<(Vec<u64>, usize), usize>) -> usize {

    if reps == 0 {
        return data.len();
    }

    if let Some(e) = cache.get(&(data.clone(), reps)) {
        return e.clone();
    }

    let mut ret = 0;
    for &n in data {
        if n == 0 {
            ret += expand(&vec![1], reps - 1, cache);
        } else {
            let s = n.to_string();
            let l = s.len();
            if s.len() % 2 == 0 {
                ret += expand(&vec![u64::from_str_radix(&s[..l/2], 10).unwrap()], reps - 1, cache);
                ret += expand(&vec![u64::from_str_radix(&s[l/2..], 10).unwrap()], reps - 1, cache);
            } else {
                ret += expand(&vec![n * 2024], reps - 1, cache);
            }
        }
    }

    cache.insert((data.clone(), reps), ret);
    ret
}

fn run(title: &str, input: &str) {
    let mut data: Vec<u64> = input.trim().split_whitespace().map(|n| n.parse().unwrap()).collect_vec();

    // for i in 0..6 {
    //     // println!("{:?}", data);
    //     println!("{} {}", i, data.len());

    //     let mut new = Vec::new();
    //     for n in data {
    //         if n == 0 {
    //             new.push(1);
    //         } else {
    //             let s = n.to_string();
    //             let l = s.len();
    //             if s.len() % 2 == 0 {
    //                 new.push(u64::from_str_radix(&s[..l/2], 10).unwrap());
    //                 new.push(u64::from_str_radix(&s[l/2..], 10).unwrap());
    //             } else {
    //                 new.push(n * 2024);
    //             }
    //         }
    //     }

    //     data = new;
    // }

    let mut cache = HashMap::new();

    println!("{} part 1: {}", title, expand(&data, 25, &mut cache));

    println!("{} part 2: {}", title, expand(&data, 75, &mut cache));
}

const INPUT_DEMO: &str = "125 17";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("11/input.txt").unwrap());
}
