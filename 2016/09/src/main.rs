use core::panic;

enum State {
    Normal,
    Len,
    Reps,
    Data,
}

fn run(title: &str, input: &str) {
    for line in input.lines() {
        let mut out = Vec::new();

        let mut state = State::Normal;
        let mut len = 0;
        let mut reps = 0;
        let mut data = Vec::new();

        for c in line.chars() {
            match state {
                State::Normal => {
                    match c {
                        '(' => {
                            len = 0;
                            reps = 0;
                            data.clear();
                            state = State::Len;
                        }
                        _ => out.push(c)
                    }
                },
                State::Len => {
                    match c {
                        '0'..='9' => {
                            len = len * 10 + c.to_digit(10).unwrap();
                        }
                        'x' => state = State::Reps,
                        _ => panic!()
                    }
                },
                State::Reps => {
                    match c {
                        '0'..='9' => {
                            reps = reps * 10 + c.to_digit(10).unwrap();
                        }
                        ')' => state = State::Data,
                        _ => panic!()
                    }

                },
                State::Data => {
                    data.push(c);

                    if data.len() == len as usize {
                        for i in 0..reps {
                            out.append(&mut data.clone());
                        }
                        state = State::Normal;
                    }
                },
            }
        }

        // println!("{} {}", out.len(), out.iter().collect::<String>());
        println!("part 1: {}", out.len());
    }
}

fn decompress(input: Vec<char>) -> Vec<char> {
    let mut out = Vec::new();

    let mut state = State::Normal;
    let mut len = 0;
    let mut reps = 0;
    let mut data = Vec::new();

    for c in input {
        match state {
            State::Normal => {
                match c {
                    '(' => {
                        len = 0;
                        reps = 0;
                        data.clear();
                        state = State::Len;
                    }
                    _ => out.push(c)
                }
            },
            State::Len => {
                match c {
                    '0'..='9' => {
                        len = len * 10 + c.to_digit(10).unwrap();
                    }
                    'x' => state = State::Reps,
                    _ => panic!()
                }
            },
            State::Reps => {
                match c {
                    '0'..='9' => {
                        reps = reps * 10 + c.to_digit(10).unwrap();
                    }
                    ')' => state = State::Data,
                    _ => panic!()
                }

            },
            State::Data => {
                data.push(c);

                if data.len() == len as usize {
                    let dec = decompress(data.clone());
                    for i in 0..reps {
                        out.append(&mut dec.clone());
                    }
                    state = State::Normal;
                }
            },
        }
    }

    out
}

fn run2(title: &str, input: &str) {
    for line in input.lines() {

        // println!("{} {}", out.len(), out.iter().collect::<String>());
        let out = decompress(line.chars().collect());
        println!("{}", out.len());
        if out.len() < 100 {
            println!("{}", out.iter().collect::<String>());
        }
    }

    println!("{} part 2: {}", title, "TODO");
}

fn decompress3(input: Vec<char>) -> usize {
    let mut out = 0;

    let mut state = State::Normal;
    let mut len = 0;
    let mut reps = 0;
    let mut data = Vec::new();

    for c in input {
        match state {
            State::Normal => {
                match c {
                    '(' => {
                        len = 0;
                        reps = 0;
                        data.clear();
                        state = State::Len;
                    }
                    _ => out += 1,
                }
            },
            State::Len => {
                match c {
                    '0'..='9' => {
                        len = len * 10 + c.to_digit(10).unwrap();
                    }
                    'x' => state = State::Reps,
                    _ => panic!()
                }
            },
            State::Reps => {
                match c {
                    '0'..='9' => {
                        reps = reps * 10 + c.to_digit(10).unwrap();
                    }
                    ')' => state = State::Data,
                    _ => panic!()
                }

            },
            State::Data => {
                data.push(c);

                if data.len() == len as usize {
                    let dec = decompress3(data.clone());
                    out += dec * reps as usize;
                    state = State::Normal;
                }
            },
        }
    }

    out
}

fn run3(title: &str, input: &str) {
    for line in input.lines() {

        // println!("{} {}", out.len(), out.iter().collect::<String>());
        let out = decompress3(line.chars().collect());
        println!("{} part 2: {}", title, out);
    }

    println!("{} part 2: {}", title, "TODO");
}



const INPUT_DEMO: &str = "ADVENT
A(1x5)BC
(3x3)XYZ
A(2x2)BCD(2x2)EFG
(6x1)(1x3)A
X(8x2)(3x3)ABCY
";

const INPUT_DEMO2: &str = "(3x3)XYZ
X(8x2)(3x3)ABCY
(27x12)(20x12)(13x14)(7x10)(1x12)A
(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN
";

fn main() {
    // run2("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("09/input.txt").unwrap());
    run2("demo 2", INPUT_DEMO2);
    run3("demo 2", INPUT_DEMO2);
    run3("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
