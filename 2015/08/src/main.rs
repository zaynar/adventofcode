use fancy_regex::{Captures, Regex};

fn run(title: &str, input: &str) {
    let mut part1 = 0;

    let re = Regex::new(r#"(\\\\)|(\\")|\\x([a-f0-9][a-f0-9])"#).unwrap();
    for line in input.lines() {
        let unesc = re.replace_all(&line[1..line.len()-1], |m: &Captures| {
            // println!("<{:?}>", m.get(0).unwrap());
            if m.get(1).is_some() {
                "\\".to_owned()
            } else if m.get(2).is_some() {
                "\"".to_owned()
            } else if let Some(c) = m.get(3) {
                (u8::from_str_radix(c.as_str(), 16).unwrap() as char).to_string()
            } else {
                panic!()
            }
        });
        // println!("{} <{}>", line, unesc);
        // assert_eq!(unesc, unescaper::unescape(&line[1..line.len()-1]).unwrap());
        part1 += line.chars().count() - unesc.chars().count();
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;

    for line in input.lines() {
        part2 += 2 + line.chars().filter(|&c| c == '"' || c == '\\').count();
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
