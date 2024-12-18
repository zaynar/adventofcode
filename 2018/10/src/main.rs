// Part 1: 10 mins
// Part 1+2: 10 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            (
                line[10..16].trim().parse().unwrap(),
                line[18..24].trim().parse().unwrap(),
                line[36..38].trim().parse().unwrap(),
                line[40..42].trim().parse().unwrap(),
            )
        })
        .collect();

    // println!("{:#?}", data);

    let mut best = (i64::MAX, 0);
    for t in 0..20_000 {
        let score =
        (data.iter().map(|(x, y, vx, vy)| x + vx*t).max().unwrap() - data.iter().map(|(x, y, vx, vy)| x + vx*t).min().unwrap()) as i64 *
        (data.iter().map(|(x, y, vx, vy)| y + vy*t).max().unwrap() - data.iter().map(|(x, y, vx, vy)| y + vy*t).min().unwrap()) as i64;
        best = best.min((score, t));
    }

    println!("{:?}", best);

    let t = best.1;

    for gy in (data.iter().map(|(x, y, vx, vy)| y + vy*t).min().unwrap() ..= data.iter().map(|(x, y, vx, vy)| y + vy*t).max().unwrap()) {
        for gx in (data.iter().map(|(x, y, vx, vy)| x + vx*t).min().unwrap() ..= data.iter().map(|(x, y, vx, vy)| x + vx*t).max().unwrap()) {
            print!("{}", if data.iter().any(|(x, y, vx, vy)| x+vx*t == gx && y+vy*t == gy) { '#' } else { '.' });
        }
        println!();
    }
}

fn main() {
    run("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
