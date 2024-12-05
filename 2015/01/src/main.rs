fn run(title: &str, input: &str) {

    println!("{} part 1: {}", title, input.chars().filter(|&c| c == '(').count() - input.chars().filter(|&c| c == ')').count());

    let mut level = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            level += 1;
        }
        if c == ')' {
            level -= 1;
        }
        if level < 0 {
            println!("{} part 2: {}", title, i + 1);
            break;
        }
    }
}

fn main() {
    run("input", &std::fs::read_to_string("01/input.txt").unwrap());
}
