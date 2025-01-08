// Part 1: 8 mins
// Part 1+2: 9 mins

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (stack, moves) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];

    let grid = Grid::from(stack);
    grid.for_each(|x, y, c| {
        match c {
            'A'..='Z' => stacks[1 + (x as usize - 1) / 4].push(*c),
            _ => ()
        }
    });

    for s in &mut stacks {
        s.reverse();
    }

    // println!("{:?}", stacks);

    for m in moves.lines() {
        let s = m.split_ascii_whitespace().collect_vec();
        let n: usize = s[1].parse().unwrap();
        let from: usize = s[3].parse().unwrap();
        let to: usize = s[5].parse().unwrap();
        for i in 0..n {
            let t = stacks[from].pop().unwrap();
            stacks[to].push(t);
        }

        // Part 2:
        let start = stacks[to].len() - n;
        stacks[to][start..].reverse();

        // println!("{:?}", stacks);
    }

    // println!("{:?}", stacks);

    println!("{} part 1: {}", title, stacks.iter().filter_map(|s| s.last()).join(""));

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
