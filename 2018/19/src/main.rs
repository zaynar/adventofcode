// Part 1: 9 mins
// Part 1+2: 32 mins

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Ban,
    Bor,
    Set,
    Gt,
    Eq,
}
const I: bool = false;
const R: bool = true;

fn consistent(eg: &(Vec<i32>, Vec<i32>, Vec<i32>), op: &(Op, bool, bool)) -> bool {
    let (before, code, after) = eg;

    let mut regs = before.clone();

    let a = if op.1 { before[code[1] as usize] } else { code[1] };
    let b = if op.2 { before[code[2] as usize] } else { code[2] };

    regs[code[3] as usize] = match op.0 {
        Op::Add => a + b,
        Op::Mul => a * b,
        Op::Ban => a & b,
        Op::Bor => a | b,
        Op::Set => a,
        Op::Gt => if a > b { 1 } else { 0 },
        Op::Eq => if a == b { 1 } else { 0 },
    };

    return regs == *after;
}

fn p(code: &((Op, bool, bool), i32, i32, i32)) {
    let op = code.0.clone();
    let a = if op.1 { format!("r{}", code.1) } else { format!("{}", code.1) };
    let b = if op.2 { format!("r{}", code.2) } else { format!("{}", code.2) };
    let c = format!("r{}", code.3);

    match op.0 {
        Op::Add => println!("{} = {} + {};", c, a, b),
        Op::Mul => println!("{} = {} * {};", c, a, b),
        Op::Ban => println!("{} = {} & {};", c, a, b),
        Op::Bor => println!("{} = {} | {};", c, a, b),
        Op::Set => println!("{} = {};", c, a),
        Op::Gt => println!("{} = {} > {};", c, a, b),
        Op::Eq => println!("{} = {} == {};", c, a, b),
    };
}

fn run(title: &str, input: &str) {
    let mut lines = input.lines();
    let ip = lines.next().unwrap()[4..].parse::<usize>().unwrap();

    let mut instr = Vec::new();
    for line in lines {
        let (op, a, b, c) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let op = match op {
            "addr" => (Op::Add, R, R),
            "addi" => (Op::Add, R, I),
            "mulr" => (Op::Mul, R, R),
            "muli" => (Op::Mul, R, I),
            "banr" => (Op::Ban, R, R),
            "bani" => (Op::Ban, R, I),
            "borr" => (Op::Bor, R, R),
            "bori" => (Op::Bor, R, I),
            "setr" => (Op::Set, R, I),
            "seti" => (Op::Set, I, I),
            "gtir" => (Op::Gt, I, R),
            "gtri" => (Op::Gt, R, I),
            "gtrr" => (Op::Gt, R, R),
            "eqir" => (Op::Eq, I, R),
            "eqri" => (Op::Eq, R, I),
            "eqrr" => (Op::Eq, R, R),
            _ => panic!()
        };
        instr.push((op, a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap(), c.parse::<i32>().unwrap()));
    }

    println!("{:?}", instr);

    let mut regs = vec![0_i32; 6];

    for code in &instr {
        p(code);
    }
    println!("\n");
    // return;

    regs[0] = 1; // part 2

    let mut i: i64 = 0;
    while regs[ip] < instr.len() as i32 {
        let code = &instr[regs[ip] as usize];

        print!("{:2} {:?} ", regs[ip], regs);
        p(code);

        let op = code.0.clone();
        let a = if op.1 { regs[code.1 as usize] } else { code.1 };
        let b = if op.2 { regs[code.2 as usize] } else { code.2 };

        regs[code.3 as usize] = match op.0 {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Ban => a & b,
            Op::Bor => a | b,
            Op::Set => a,
            Op::Gt => if a > b { 1 } else { 0 },
            Op::Eq => if a == b { 1 } else { 0 },
        };
        regs[ip] += 1;

        if regs[ip] == 3 {

            for r1 in 1..=regs[2] {
                if regs[2] % r1 == 0 {
                    regs[0] += r1;
                }
            }

            println!("{} part 2: {:?}", title, regs[0]);
            break;
        }

        i += 1;
        if i == 100 {
            break;
        }
        if i % 1_000_000 == 0 {
            println!("{} {:?}", i, regs);
        }
    }

    println!("{} part 1: {:?}", title, regs);
}

const INPUT_DEMO: &str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
