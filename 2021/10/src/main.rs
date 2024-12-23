// Part 1: 6 mins
// Part 1+2: 9 mins

fn opposite(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Bad {}", c)
    }
}

fn run(title: &str, input: &str) {
    let mut part1 = 0;

    let mut scores = vec![];
    'L: for line in input.lines() {
        let mut stack = vec![];
        'C: for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let p = stack.pop().unwrap();
                    if c != opposite(p) {
                        // println!("Got {}, expected {}", c, opposite(p));
                        part1 += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!(),
                        };
                        continue 'L;
                    }
                }
                _ => panic!(),
            }
        }

        let mut score: u64 = 0;
        while let Some(p) = stack.pop() {
            score = score * 5 + match p {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            };
        }

        // println!("{}", score);
        scores.push(score);
    }

    println!("{} part 1: {}", title, part1);

    scores.sort();
    println!("{} part 2: {}", title, scores[scores.len() / 2]);
}

const INPUT_DEMO: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
