use itertools::Itertools;

fn next_row(input: &str) -> String {
    (".".to_owned() + input + ".").chars().tuple_windows().map(|t| {
        match t {
            ('^', '^', '.') |
            ('.', '^', '^') |
            ('^', '.', '.') |
            ('.', '.', '^') => '^',
            _ => '.'
        }
    }).collect()
}

fn run(title: &str, input: &str, rows: u32) {

    let mut part1 = 0;
    let mut row = input.to_owned();
    for i in 0..rows {
        // println!("[{}]", row);
        part1 += row.chars().filter(|c| *c == '.').count();
        row = next_row(&row);
    }

    println!("{} part 1: {}", title, part1);

}

const INPUT_DEMO: &str = "";

fn main() {
    run("demo", ".^^.^.^^^^", 10);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap().trim(), 40);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap().trim(), 400_000);
}
