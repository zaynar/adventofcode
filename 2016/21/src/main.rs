use itertools::Itertools;

#[derive(Debug)]
enum Step {
    SwapPos(usize, usize),
    SwapChar(char, char),
    RotateL(usize),
    RotateR(usize),
    RotateChar(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

peg::parser! {
    grammar input_parser() for str {
        rule num() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule letter() -> char
            = ['a'..='z']

        rule step() -> Step
            = "swap position " x:num() " with position " y:num() "\n" { Step::SwapPos(x, y) }
            / "swap letter " x:letter() " with letter " y:letter() "\n" { Step::SwapChar(x, y) }
            / "rotate left " x:num() (" steps\n" / " step\n") { Step::RotateL(x) }
            / "rotate right " x:num() (" steps\n" / " step\n") { Step::RotateR(x) }
            / "rotate based on position of letter " x:letter() "\n" { Step::RotateChar(x) }
            / "reverse positions " x:num() " through " y:num() "\n" { Step::Reverse(x, y) }
            / "move position " x:num() " to position " y:num() "\n" { Step::Move(x, y) }

        pub rule file() -> Vec<Step>
            = step()+
    }
}

fn run(title: &str, input: &str, msg: &str) {
    let data = input_parser::file(input).unwrap();
    println!("{:?}", data);

    let mut msg = msg.chars().collect_vec();
    let msg = msg.as_mut_slice();

    for rule in &data {
        match rule {
            &Step::SwapPos(x, y) => (msg[x], msg[y]) = (msg[y], msg[x]),
            &Step::SwapChar(x, y) => msg.iter_mut().for_each(|c| if *c == x { *c = y } else if *c == y { *c = x }),
            &Step::RotateL(x) => msg.rotate_left(x % msg.len()),
            &Step::RotateR(x) => msg.rotate_right(x % msg.len()),
            &Step::RotateChar(x) => {
                let idx = msg.iter().find_position(|c| **c == x).unwrap().0;
                // println!("- {} {}", idx, (idx + 1 + if idx >= 4 { 1 } else { 0 }));
                msg.rotate_right((idx + 1 + if idx >= 4 { 1 } else { 0 }) % msg.len());
            },
            &Step::Reverse(x, y) => msg[x..=y].reverse(),
            &Step::Move(x, y) => if x < y { msg[x..=y].rotate_left(1) } else { msg[y..=x].rotate_right(1) },
        }
        // println!("{}", msg.iter().collect::<String>());
    }

    println!("{} part 1: {}", title, msg.iter().collect::<String>());
}

fn run2(title: &str, input: &str, msg: &str) {
    let data = input_parser::file(input).unwrap();

    let mut msg = msg.chars().collect_vec();
    let msg = msg.as_mut_slice();

    for rule in data.iter().rev() {
        match rule {
            &Step::SwapPos(x, y) => (msg[x], msg[y]) = (msg[y], msg[x]),
            &Step::SwapChar(x, y) => msg.iter_mut().for_each(|c| if *c == x { *c = y } else if *c == y { *c = x }),
            &Step::RotateL(x) => msg.rotate_right(x % msg.len()),
            &Step::RotateR(x) => msg.rotate_left(x % msg.len()),
            &Step::RotateChar(x) => {
                for i in 0.. {
                    let mut msg2 = msg.to_vec();
                    msg2.rotate_left(i);

                    let idx = msg2.iter().find_position(|c| **c == x).unwrap().0;
                    msg2.rotate_right((idx + 1 + if idx >= 4 { 1 } else { 0 }) % msg.len());

                    if msg2 == msg {
                        msg.rotate_left(i);
                        break;
                    }

                }
            },
            &Step::Reverse(x, y) => msg[x..=y].reverse(),
            &Step::Move(x, y) => if x < y { msg[x..=y].rotate_right(1) } else { msg[y..=x].rotate_left(1) },
        }
        // println!("{}", msg.iter().collect::<String>());
    }

    println!("{} part 2: {}", title, msg.iter().collect::<String>());
}


const INPUT_DEMO: &str = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 steps
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d
";

fn main() {
    run("demo", INPUT_DEMO, "abcde");
    run("input", &std::fs::read_to_string("21/input.txt").unwrap(), "abcdefgh");
    run2("input", &std::fs::read_to_string("21/input.txt").unwrap(), "fbgdceah");
}
