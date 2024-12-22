// Part 1: 8 mins
// Part 1+2: 16 mins

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule paren() -> i64
            = "(" n:expr() ")" { n }

        rule simple() -> i64
            = number()
            / paren()

        #[cache_left_rec]
        pub rule expr() -> i64
            = a:expr() " + " b:simple() { a + b }
            / a:expr() " * " b:simple() { a * b }
            / simple()


        rule paren2() -> i64
            = "(" n:expr2() ")" { n }

        rule simple2() -> i64
            = number()
            / paren2()

        #[cache_left_rec]
        pub rule expr2() -> i64
            = a:expr2() " * " b:expr3() { a * b }
            / expr3()

        #[cache_left_rec]
        pub rule expr3() -> i64
            = a:expr3() " + " b:simple2() { a + b }
            / simple2()

    }
}

fn run(title: &str, input: &str) {
    let mut part1 = 0;
    for line in input.lines() {
        let v = input_parser::expr(line).unwrap();
        // println!("{}", v);
        part1 += v;
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;
    for line in input.lines() {
        let v = input_parser::expr2(line).unwrap();
        // println!("{}", v);
        part2 += v;
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap());
}
