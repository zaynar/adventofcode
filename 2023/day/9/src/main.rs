use std::fs;

fn part1(vals: Vec<i32>) -> i32 {
    if vals.iter().all(|&v| v == 0) {
        return 0;
    }

    let newvals: Vec<_> = vals.iter().zip(vals[1..].iter()).map(|(a, b)| b - a).collect();
    println!(" {:?}", newvals);
    return vals.last().unwrap() + part1(newvals);
}

fn main() {
    let mut sum = 0;
    for record in fs::read_to_string("input").unwrap().lines() {
        let mut vals: Vec<i32> = record.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
        vals.reverse(); // for part 2 only
        println!("{:?}", vals);
        let n = part1(vals);
        println!("= {:?}", n);
        sum += n;
    }
    println!("Answer: {}", sum);
}
