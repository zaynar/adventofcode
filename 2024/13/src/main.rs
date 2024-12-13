// Part 1: 9 mins
// Part 1+2: 26 mins

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule button() -> (i64, i64)
            = "X+" x:number() ", Y+" y:number() "\n" { (x, y) }

        rule prize() -> (i64, i64)
            = "X=" x:number() ", Y=" y:number() "\n" { (x, y) }

        rule machine() -> Machine
            = "Button A: " a:button() "Button B: " b:button() "Prize: " prize:prize() { Machine { a, b, prize }}

        pub rule file() -> Vec<Machine>
            = machine() ++ "\n"
    }
}

fn run(title: &str, input: &str, p2: bool) {
    let data = input_parser::file(input).unwrap();

    // println!("{:?}", data);

    let mut sum = 0;
    for m in &data {
        let px = m.prize.0 + if p2 { 10000000000000 } else { 0 };
        let py = m.prize.1 + if p2 { 10000000000000 } else { 0 };

        // px = na ax + nb bx
        // py = na ay + nb by

        // nb = (py - na ay) / by   if by != 0

        // px = na ax + bx (py - na ay) / by
        // px by = na ax by + bx py - bx na ay
        // px by - bx py = na(ax by - bx ay)

        let na = (px * m.b.1 - py * m.b.0) / (m.a.0 * m.b.1 - m.b.0 * m.a.1);

        let tx = m.a.0 * na;
        let ty = m.a.1 * na;
        let nb = (px - tx) / m.b.0;
        if tx + m.b.0 * nb == px && ty + m.b.1 * nb == py {
            sum += na * 3 + nb;
        }
    }

    println!("{} part N: {}", title, sum);
}


const INPUT_DEMO: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() {
    run("demo", INPUT_DEMO, false);
    run("input", &std::fs::read_to_string("13/input.txt").unwrap(), false);
    run("input", &std::fs::read_to_string("13/input.txt").unwrap(), true);
}
