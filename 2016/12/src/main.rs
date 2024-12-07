use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut pc: i32 = 0;
    let lines = input.lines().collect_vec();
    let mut regs: HashMap<&str, i32> = HashMap::new();
    regs.insert("a", 0);
    regs.insert("b", 0);
    regs.insert("c", 0);
    regs.insert("d", 0);

    while pc < lines.len() as i32 {
        let (op, r) = lines[pc as usize].split_once(" ").unwrap();
        // println!("{} {:?} {:?}", pc, regs, lines[pc as usize]);

        match op {
            "cpy" => {
                let (x, y) = r.split_once(" ").unwrap();
                regs.insert(y, if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() });
                pc += 1;
            }
            "inc" => {
                regs.insert(r, regs[r] + 1);
                pc += 1;
            }
            "dec" => {
                regs.insert(r, regs[r] - 1);
                pc += 1;
            }
            "jnz" => {
                let (x, y) = r.split_once(" ").unwrap();
                let x = if regs.contains_key(x) { regs[x] } else { x.parse().unwrap() };
                let y = if regs.contains_key(y) { regs[y] } else { y.parse().unwrap() };
                if x != 0 {
                    pc += y;
                } else {
                    pc += 1;
                }
            }
            _ => panic!()
        }
    }

    println!("{:?}", regs);

    println!("{} part N: {}", title, regs["a"]);
}

const INPUT_DEMO: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
