// Part 1: 22 mins
// Part 1+2: 74 mins

use std::collections::HashSet;

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

fn p(code: &((Op, bool, bool), i64, i64, i64)) {
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
        instr.push((op, a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap(), c.parse::<i64>().unwrap()));
    }

    println!("{:?}", instr);

    for (i, code) in instr.iter().enumerate() {
        print!("L{}: ", i);
        p(code);
    }
    println!("\n");
    // return;

    let mut best = 10_000;

    'OUTER: for j in 0.. {
        if j % 1000 == 0 {
            println!("{}...", j);
        }

        let mut regs = vec![0_i64; 6];
        regs[0] = j;

        let mut i: i64 = 0;
        while regs[ip] < instr.len() as i64 {
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

            // if op.0 == Op::Eq && code.2 == 0 {
            //     println!("{} part 1: {}", title, a);
            //     return;
            // }

            i += 1;
            if i == 200 { return; }
            if i >= best {
                continue 'OUTER;
            }
        }

        best = i;
        println!("{} part 1: j={} i={}", title, j, i);
    }
}

fn run2() {
    for i in 0.. {
        let mut r0 = 0_i64;
        let mut r2 = 0_i64;
        let mut r3 = 0_i64;
        let mut r4 = 0_i64;
        let mut r5 = 0_i64;

        r0 = i;

        r5 = 0;
        loop {
            r4 = r5 | (1 << 16);

            r5 = 3935295;

            'L8: loop {
                // println!("a {} _ {} {} r4={} {}", r0, r2, r3, r4, r5);
                r2 = r4 & 255;
                r5 = (((r5 + r2) & 16777215) * 65899) & 16777215;

                if r4 >= 256 {

                    r2 = 0;
                    loop {
                        // println!("b {} _ {} {} {} {}", r0, r2, r3, r4, r5);
                        r3 = (r2+1)*256;

                        if r3 > r4 {
                            r4 = r2;
                            continue 'L8;
                        }
                        r2 = r2 + 1;
                    }
                }
                break;
            }

            println!("### {}", r5);
            return;
            if r5 == r0 {
                println!("### halt {}", i);
                return;
            }
        }
    }
}

fn run3() {
    let mut shortest = (i64::MAX, 0);
    let mut longest = (0, 0);

    'OUTER: for i in 0.. {
    // 'OUTER: for i in [16457176, 1589502, 11856231] {
        if i % 1000 == 0 {
            println!("{}...", i);
        }
        let mut r0 = 0_i64;
        let mut r5 = 0_i64;

        r0 = i;

        let mut seen = HashSet::new();

        let mut instr = 0;

        let mut prev_r5 = 0;
        // let mut min = None;

        r5 = 0;
        loop {
            // println!("X {} _ {} {} r4={} {}", r0, r2, r3, r4, r5);
            let mut r4 = r5 | (1 << 16);

            r5 = 3935295;

            instr += 2;

            // let mut seen = HashSet::new();
            'L8: loop {
                // if !seen.insert((r4, r5)) {
                //     println!("# inner loop");
                //     continue 'OUTER;
                // }
                // println!("seen {}", seen.len());

                // println!("a {} _ {} {} r4={} {}", r0, r2, r3, r4, r5);
                let mut r2 = r4 & 255;
                r5 = (((r5 + r2) & 16777215) * 65899) & 16777215;

                instr += 6;

                if !(256 > r4) {

                    // not correct?
                    r2 = (r4 + 0xff) >> 8;
                    r4 = r2;

                    instr += 6 * r2;

                } else {
                    break;
                }
            }

            //println!("### {}", r5);
            //return;
            if r5 == r0 {
                longest = longest.max((instr, i));
                shortest = shortest.min((instr, i));
                // println!("Z {} _ {} {} r4={} {}", r0, r2, r3, r4, r5);
                // println!("### halt {}", i);
                println!("l={:?} s={:?}", longest, shortest);
                // return;
                continue 'OUTER;
            }

            if !seen.insert(r5)
            {
                println!("LOOPED {} {}", prev_r5, r5);
                return;

            }
            prev_r5 = r5;

        }
    }
}

fn run4(title: &str, input: &str) {
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
        instr.push((op, a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap(), c.parse::<i64>().unwrap()));
    }

    println!("{:?}", instr);

    for (i, code) in instr.iter().enumerate() {
        print!("L{}: ", i);
        p(code);
    }
    println!("\n");
    // return;


    'OUTER: for j in 0.. {
        if j % 1000 == 0 {
            println!("{}...", j);
        }

        let mut seen = HashSet::new();

        let mut regs = vec![0_i64; 6];
        regs[0] = j;

        let mut i: i64 = 0;
        while regs[ip] < instr.len() as i64 {
            let code = &instr[regs[ip] as usize];

            // print!("{:2} {:?} ", regs[ip], regs);
            // p(code);

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

            if regs[ip] == 28 {
                // println!("{} part 1: {}", title, regs[5]);
                // return;

                println!("# {}", regs[5]);
                if !seen.insert(regs[5]) {
                    println!("{} part 2: prev one (not {})", title, regs[5]);
                    return;
                }
            }
        }
    }
}


fn main() {
    // run("input", &std::fs::read_to_string("21/input.txt").unwrap());
    // run3();
    run4("input", &std::fs::read_to_string("21/input.txt").unwrap());
}
