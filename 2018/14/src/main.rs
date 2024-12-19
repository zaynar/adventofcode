// Part 1: 8 mins
// Part 1+2: 11 mins

use itertools::Itertools;

fn run(title: &str, input: usize) {
    let mut b = vec![3, 7];
    let mut e0 = 0;
    let mut e1 = 1;

    while b.len() < input + 10 {
        let sum = b[e0] + b[e1];
        if sum >= 10 {
            b.push(sum / 10);
            b.push(sum % 10);
        } else {
            b.push(sum);
        }

        e0 = (e0 + 1 + b[e0]) % b.len();
        e1 = (e1 + 1 + b[e1]) % b.len();

        // println!("{:?}", b);
    }

    let mut part1 = b[input as usize..(input as usize+10)].iter().map(|n| n.to_string()).collect::<String>();

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, "TODO");
}

fn run2(title: &str, input: usize) {
    let mut b = vec![3, 7];
    let mut e0 = 0;
    let mut e1 = 1;

    let input = input.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).collect_vec();

    loop {
        let sum = b[e0] + b[e1];
        if sum >= 10 {
            b.push(sum / 10);

            if b.ends_with(&input) {
                println!("{} part 2: {}", title, b.len() - input.len());
                break;
            }

            b.push(sum % 10);
        } else {
            b.push(sum);
        }

        if b.ends_with(&input) {
            println!("{} part 2: {}", title, b.len() - input.len());
            break;
        }

        e0 = (e0 + 1 + b[e0]) % b.len();
        e1 = (e1 + 1 + b[e1]) % b.len();

        // println!("{:?}", b);
    }
}

fn main() {
    run("demo", 2018);
    run("input", 598701);
    run2("demo", 59414);
    run2("input", 598701);
}
