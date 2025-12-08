// Part 1: 4 mins
// Part 1+2: 20 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut lines = input.lines().collect_vec();
    let ops = lines.pop().unwrap().split_ascii_whitespace().collect_vec();
    let nums = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut part1 = 0;

    for (i, op) in ops.iter().enumerate() {
        part1 += match *op {
            "*" => nums.iter().map(|n| n[i]).product::<u64>(),
            "+" => nums.iter().map(|n| n[i]).sum::<u64>(),
            _ => panic!(),
        }
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;

    let mut lines = input.lines().collect_vec();
    let ops = lines.pop().unwrap().to_owned() + "         #";
    let opind = ops.char_indices().filter(|(i, op)| *op != ' ');
    let linechars = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    for ((i0, op0), (i1, op1)) in opind.tuple_windows() {
        // println!("{i0} {op0} - {i1}");
        let mut ns = vec![];
        for i in (i0..i1 - 1).rev() {
            let n = linechars
                .iter()
                .map(|line| line.get(i).unwrap_or(&' '))
                .collect::<String>();
            if !n.trim().is_empty() {
                ns.push(n.trim().parse::<u64>().unwrap());
            }
        }
        // println!("# {ns:?}");

        part2 += match op0 {
            '*' => ns.iter().product::<u64>(),
            '+' => ns.iter().sum::<u64>(),
            _ => panic!(),
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("06/input.txt").unwrap());
}
