#[derive(Debug)]
enum Step {
    On(u32, u32, u32, u32),
    Off(u32, u32, u32, u32),
    Toggle(u32, u32, u32, u32),
}

peg::parser! {
    grammar input_parser() for str {
        rule num() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule on() -> Step
            = "turn on " a:num() "," b:num() " through " c:num() "," d:num() "\n" { Step::On(a, b, c, d) }

        rule off() -> Step
            = "turn off " a:num() "," b:num() " through " c:num() "," d:num() "\n" { Step::Off(a, b, c, d) }

        rule toggle() -> Step
            = "toggle " a:num() "," b:num() " through " c:num() "," d:num() "\n" { Step::Toggle(a, b, c, d) }

        rule step() -> Step
            = on() / off() / toggle()

        pub rule file() -> Vec<Step>
            = step()+
    }
}

fn set<F: Fn(&mut u8)>(grid: &mut Vec<u8>, a: u32, b: u32, c: u32, d: u32, f: F) {
    for y in b..=d {
        for x in a..=c {
            f(grid.get_mut((x + y * 1000) as usize).unwrap());
        }
    }
}

fn run(title: &str, input: &str) {
    let data = input_parser::file(input).unwrap();

    println!("{:?}", data);

    let mut grid = vec![0; 1000000];
    for step in &data {
        match *step {
            Step::On(a, b, c, d) => set(&mut grid, a, b, c, d, |n| *n = 1),
            Step::Off(a, b, c, d) => set(&mut grid, a, b, c, d, |n| *n = 0),
            Step::Toggle(a, b, c, d) => set(&mut grid, a, b, c, d, |n| *n = 1 - *n),
        }
    }

    println!("{} part 1: {}", title, grid.iter().map(|&i| i as u32).sum::<u32>());

    let mut grid = vec![0; 1000000];
    for step in &data {
        match *step {
            Step::On(a, b, c, d) => set(&mut grid, a, b, c, d, |n| *n += 1),
            Step::Off(a, b, c, d) => set(&mut grid, a, b, c, d, |n| *n = n.saturating_sub(1)),
            Step::Toggle(a, b, c, d) => set(&mut grid, a, b, c, d, |n| *n += 2),
        }
    }

    println!("{} part 2: {}", title, grid.iter().map(|&i| i as u32).sum::<u32>());

}

fn main() {
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
