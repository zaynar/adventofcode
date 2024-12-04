use std::collections::HashMap;

use itertools::Itertools;

fn phase(input: Vec<i32>) -> Vec<i32> {
    (0..input.len()).map(|i| {

        (0..input.len()).map(|j| {
            let pat = [0, 1, 0, -1][((j + 1) / (i + 1)) % 4];
            input[j] * pat
        }).sum::<i32>().abs() % 10
    }).collect()
}

fn phase2(memo: &mut HashMap<(i32, usize), i32>, input: &Vec<i32>, phases: i32, idx: usize) -> i32 {
    if idx >= input.len() {
        return 0;
    }

    if phases == 0 {
        return input[idx];
    }

    if let Some(sum) = memo.get(&(phases, idx)) {
        return sum.abs() % 10;
    }

    // Optimise similar ranges
    if idx > 0 {
        if let Some(ret) = memo.get(&(phases, idx - 1)) {
            let mut sum = *ret;

            let mut j0 = idx - 1;
            let mut j1 = idx;
            while j0 < input.len() {
                for j in j0..j1 {
                    sum -= phase2(memo, input, phases - 1, j);
                }
                for j in j0+(idx-1+1)..j1+(idx+1) {
                    sum += phase2(memo, input, phases - 1, j);
                }

                for j in j0+(idx-1+1)*2..j1+(idx+1)*2 {
                    sum += phase2(memo, input, phases - 1, j);
                }
                for j in j0+(idx-1+1)*3..j1+(idx+1)*3 {
                    sum -= phase2(memo, input, phases - 1, j);
                }

                j0 += (idx-1+1)*4;
                j1 += (idx+1)*4;
            }
            memo.insert((phases, idx), sum);
            return sum.abs() % 10;
        }
    }

    let mut sum = 0;

    // let pat = [0, 1, 0, -1][((j + 1) / (idx + 1)) % 4];

    // Skip idx
    // Add idx+1
    // Skip idx+1
    // Sub idx+1
    // Skip idx+1
    // Loop
    let mut j0 = idx;
    while j0 < input.len() {
        // if phases > 1 { println!("{} idx={} j={}..{} - {}..{}", phases, idx, j0, j0+idx+1, j0+(idx+1)*2, j0+(idx+1)*3); }

        for j in j0..j0+idx+1 {
            sum += phase2(memo, input, phases - 1, j);
        }
        for j in j0+(idx+1)*2..j0+(idx+1)*3 {
            sum -= phase2(memo, input, phases - 1, j);
        }

        j0 += (idx+1)*4;
    }

    // if phases % 10 == 0 { println!("{} {} {}", phases, idx, sum); }
    memo.insert((phases, idx), sum);

    sum.abs() % 10
}

fn run(title: &str, input: &str) {
    let data: Vec<i32> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let mut d = data.clone();
    for i in 0..100 {
        // println!("{:?}", d);
        d = phase(d);
    }
    println!("{} part 1: {}", title, d[0..8].iter().join(""));

    let mut memo = HashMap::new();

    for i in 0..8 {
        print!("{}", phase2(&mut memo, &data, 100, i));
    }
    println!(" <--");

    // println!("{} part 2: {}", title, "TODO");
}

fn run2(title: &str, input: &str) {
    let data: Vec<i32> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let offset = usize::from_str_radix(&input[0..7], 10).unwrap();

    let data = data.repeat(10000);

    print!("{} part 2: ", title);

    let mut memo = HashMap::new();

    let mut out = Vec::new();
    for i in offset..offset+8 {
        out.push(phase2(&mut memo, &data, 100, i));
    }
    println!("{}", out.iter().join(""));
}

fn main() {
    run("demo", "80871224585914546619083218645595");
    // run("input", &std::fs::read_to_string("16/input.txt").unwrap());

    run2("demo", "03036732577212944063491565474664");
    run2("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
