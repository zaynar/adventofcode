// Part 1: 74 mins
// Part 1+2: 92 mins

use rayon::prelude::*;

use std::collections::HashMap;

use itertools::Itertools;

fn decode(title: &str, input: &str) {
    let mut n = 1;

    let mut regs = HashMap::from([
        ("w", "zero".to_owned()),
        ("x", "zero".to_owned()),
        ("y", "zero".to_owned()),
        ("z", "zero".to_owned()),
    ]);

    for line in input.lines() {
        let (op, args) = line.split_once(" ").unwrap();
        let (ar, a0, a1) = if let Some(args) = args.split_once(" ") {
            (args.0,
                regs.get(args.0).unwrap().clone(),
                regs.get(args.1).cloned().unwrap_or(args.1.to_owned()))
        } else {
            (args,
                regs.get(args).unwrap().clone(),
                "?".to_owned())
        };
        let r = format!("r{:03}", n);
        n += 1;
        regs.insert(ar, r.clone());
        let p = match op {
            "inp" => format!("{} = input.next().unwrap() as i32", r),
            "add" => format!("{} = {} + {}", r, a0, a1),
            "mul" => {
                if a1 == "0" {
                    format!("{} = 0", r)
                } else {
                    format!("{} = {} * {}", r, a0, a1)
                }
            }
            "div" => {
                if a1 == "1" {
                    format!("{} = {}", r, a0)
                } else {
                    format!("{} = {} / {}", r, a0, a1)
                }
            }
            "mod" => format!("{} = {} % {}", r, a0, a1),
            "eql" => format!("{} = if {} == {} {{ 1 }} else {{ 0 }}", r, a0, a1),
            _ => panic!(),
        };

        println!("let {};", p);
    }
}

fn run(title: &str, input: &str) {
    let mut n = 1;

    let mut regs = HashMap::from([
        ("w", 0),
        ("x", 0),
        ("y", 0),
        ("z", 0),
    ]);

    // let mut ins = [0; 14].iter();
    // let mut ins = [14,13,0,0,0,0,0,0, 0,0,0,0,0,0].iter();
    let mut ins = [9,9,9,9,9,9,9,9, 1,9,9,9,9,9].iter();

    for (i, line) in input.lines().enumerate() {
        let (op, args) = line.split_once(" ").unwrap();
        let (ar, a0, a1) = if let Some(args) = args.split_once(" ") {
            (args.0,
                regs.get(args.0).copied().unwrap(),
                regs.get(args.1).copied().unwrap_or_else(|| args.1.parse::<i64>().unwrap()))
        } else {
            (args,
                regs.get(args).copied().unwrap(),
                0)
        };
        let p = match op {
            "inp" => {
                println!("- {:?}", regs);
                let mut z = regs["z"];
                while z > 0 {
                    print!("{} ", z % 26);
                    z /= 26;
                }
                println!();

                *ins.next().unwrap()
            },
            "add" => a0 + a1,
            "mul" => a0 * a1,
            "div" => a0 / a1,
            "mod" => a0 % a1,
            "eql" => {
                if (i - 6) % 18 == 0 {
                    println!("eql {} {}", a0, a1);
                }
                if a0 == a1 { 1 } else { 0 }
            }
            _ => panic!(),
        };
        regs.insert(ar, p);
    }

    println!("{} {} {} {}", regs["x"], regs["y"], regs["z"], regs["w"]);
    let mut z = regs["z"];
    while z > 0 {
        print!("{} ", z % 26);
        z /= 26;
    }
    println!();
}

fn okay(input: &str, ins: &Vec<i64>) -> Option<Vec<i64>> {
    let mut regs = HashMap::from([
        ("w", 0),
        ("x", 0),
        ("y", 0),
        ("z", 0),
    ]);

    let mut ins = ins.iter();

    let mut exp = vec![];

    for (i, line) in input.lines().enumerate() {
        let (op, args) = line.split_once(" ").unwrap();
        let (ar, a0, a1) = if let Some(args) = args.split_once(" ") {
            (args.0,
                regs.get(args.0).copied().unwrap(),
                regs.get(args.1).copied().unwrap_or_else(|| args.1.parse::<i64>().unwrap()))
        } else {
            (args,
                regs.get(args).copied().unwrap(),
                0)
        };
        let p = match op {
            "inp" => *ins.next().unwrap(),
            "add" => a0 + a1,
            "mul" => a0 * a1,
            "div" => a0 / a1,
            "mod" => a0 % a1,
            "eql" => {
                if (i - 6) % 18 == 0 {
                    // println!("eql {} {}", a0, a1);
                    exp.push(a0);
                }
                if a0 == a1 { 1 } else { 0 }
            }
            _ => panic!(),
        };
        regs.insert(ar, p);
    }

    if regs["z"] == 0 {
        return None;
    } else {
        let mut ret = vec![];
        let mut z = regs["z"];
        while z > 0 {
            ret.push(z % 26);
            z /= 26;
        }
        return Some(ret);

        // return Some(exp);
    }
}

fn run2(title: &str, input: &str) {
    let mut n = 1;

    // 5 2 1 1 1 4

    // let digits = (1..=9).rev(); // part 1
    let digits = (1..=9); // part 2

    for key in (0..5).map(|_| digits.clone()).multi_cartesian_product() {

        // if (key[0], key[1]) < (2, 0) { continue; }
        // if (key[0], key[1]) < (7, 0) { continue; }
        // if (key[0], key[1]) < (9, 9) { break; }

        if key[1..5] != [4,1,9,8] { continue; }

        for key2 in (0..2).map(|_| digits.clone()).multi_cartesian_product() {

            let mut ins = key.clone();
            ins.append(&mut key2.clone());
            while ins.len() < 14 {
                ins.push(0);
            }

            if let Some(exp) = okay(input, &ins) {
                if exp.len() > 4 {
                    continue;
                }
                println!("C {:?} {:?}", ins, exp);

                for key2b in (0..2).map(|_| digits.clone()).multi_cartesian_product() {

                    let mut ins = key.clone();
                    ins.append(&mut key2.clone());
                    ins.append(&mut key2b.clone());
                    while ins.len() < 14 {
                        ins.push(0);
                    }

                    if let Some(exp) = okay(input, &ins) {
                        if exp.len() > 3 {
                            continue;
                        }
                        println!("A {:?} {:?}", ins, exp);

                        // for mut key3 in (0..5).map(|_| digits.clone()).multi_cartesian_product() {

                        //     let mut ins = key.clone();
                        //     ins.append(&mut key2.clone());
                        //     ins.append(&mut key2b.clone());
                        //     ins.append(&mut key3.clone());

                        //     if let Some(exp) = okay(input, &ins) {
                        //         if exp.len() < 2 {
                        //             println!("B {:?} {:?}", ins, exp);
                        //             continue;
                        //         }

                        //     } else {
                        //         println!("solved {:?}", ins);
                        //         return;
                        //     }
                        // }

                        if (0..5).map(|_| digits.clone()).multi_cartesian_product().collect_vec().par_iter().any(|key3| {

                            let mut ins = key.clone();
                            ins.append(&mut key2.clone());
                            ins.append(&mut key2b.clone());
                            ins.append(&mut key3.clone());

                            if let Some(exp) = okay(input, &ins) {
                                if exp.len() < 2 {
                                    println!("B {:?} {:?}", ins, exp);
                                    return false;
                                }

                            } else {
                                println!("solved {:?} {}", ins, ins.iter().join(""));
                                return true;
                            }

                            false
                        }) {
                            return;
                        }
                    }
                }

            } else {
                println!("solved {:?} {}", ins, ins.iter().join(""));
                break;
            }
        }

        // break;
    }
}

fn main() {
    // decode("input", &std::fs::read_to_string("24/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("24/input.txt").unwrap());

    // 99799212949967
    // 94198111816317
    // 84198111816316
    // 34198111816311
}
