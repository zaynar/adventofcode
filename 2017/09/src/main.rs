// Part 1: 10 mins
// Part 1+2: 13 mins

#[derive(Debug)]
enum Item {
    Junk(String),
    Group(Vec<Item>),
}

fn score(i: &Item, base: u32) -> u32 {
    match i {
        Item::Junk(_) => 0,
        Item::Group(vec) => base + vec.iter().map(|c| score(c, base + 1)).sum::<u32>(),
    }
}

fn score2(i: &Item) -> u32 {
    match i {
        Item::Junk(s) => {
            let mut n = 0;
            let mut x = false;
            for c in s.chars() {
                if x {
                    x = false;
                } else if c == '!' {
                    x = true;
                } else {
                    n += 1;
                }
            }
            n
        },
        Item::Group(vec) => vec.iter().map(|c| score2(c)).sum::<u32>(),
    }
}

peg::parser! {
    grammar input_parser() for str {
        rule num() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule letter() -> char
            = ['a'..='z']

        rule group() -> Item
            = "{" cs:(item() ** ",") "}" { Item::Group(cs) }

        rule junk() -> Item
            = "<" s:$(([^ '>' | '!'] / "!" [_])*) ">" { Item::Junk(s.to_owned()) }

        rule item() -> Item
            = group() / junk()

        pub rule file() -> Item
            = i:item() ![_] { i }

       }
}

fn run(title: &str, input: &str) {
    let data = input_parser::file(input.trim()).unwrap();

    // println!("{:?}", data);

    println!("{} part 1: {}", title, score(&data, 1));

    println!("{} part 2: {}", title, score2(&data));
}

const INPUT_DEMO: &str = "{{<a!>},{<a!>},{<a!>},{<ab>}}";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
