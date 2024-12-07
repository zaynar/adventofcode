use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {

    let lines = input.lines().collect_vec();

    let mut pc: i32 = 0;
    let mut regs = HashMap::from([("a", 1_i64), ("b", 0)]);

    while 0 <= pc && pc < lines.len() as i32 {
        // println!("{} {:?}", pc, regs);
        let (op, data) = lines[pc as usize].split_once(" ").unwrap();
        match op {
            "hlf" => { *regs.get_mut(&data).unwrap() /= 2; pc += 1; }
            "tpl" => { *regs.get_mut(&data).unwrap() *= 3; pc += 1; }
            "inc" => { *regs.get_mut(&data).unwrap() += 1; pc += 1; }
            "jmp" => { pc += data.parse::<i32>().unwrap(); }
            "jie" => { let (r, off) = data.split_once(", ").unwrap(); pc += if regs[r] % 2 == 0 { off.parse().unwrap() } else { 1 } }
            "jio" => { let (r, off) = data.split_once(", ").unwrap(); pc += if regs[r] == 1 { off.parse().unwrap() } else { 1 } }
            _ => panic!(),
        }
    }

    println!("{} part: {:?}", title, regs);
}

const INPUT_DEMO: &str = "inc a
jio a, +2
tpl a
inc a
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("23/input.txt").unwrap());
}
