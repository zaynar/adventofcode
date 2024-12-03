peg::parser! {
    grammar input_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']*<1,3>) {? n.parse().or(Err("number")) }

        rule ident() -> String
            = i:$(['a'..='z' | 'A'..='Z']+) { i.to_string() }

        rule mul() -> u32
            = "mul(" a:number() "," b:number() ")" { a * b }

        pub rule part1() -> u32
            = a:(mul() / ([_] { 0 }))* { a.iter().sum() }

        pub rule part2() -> u32
            = a:(
                n:mul() { (-1, n) }
                / "do()" { (1, 0) }
                / "don't()" { (0, 0) }
                / ([_] { (-1, 0) }))*
            {
                let (enabled, sum) = a.iter().fold((true, 0), |(enabled, sum), (flag, n)| {
                    match flag {
                        -1 => (enabled, if enabled { sum + n } else { sum }),
                        0 => (false, sum),
                        1 => (true, sum),
                        _ => panic!(),
                    }
                });
                sum
            }
    }
}

fn run(title: &str, input: &str) {
    println!("{} part 1: {}", title, input_parser::part1(input).unwrap());

    println!("{} part 2: {}", title, input_parser::part2(input).unwrap());
}

fn main() {
    run("demo 1", "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    run("demo 1", "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
