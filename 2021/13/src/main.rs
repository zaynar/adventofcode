// Part 1: 8 mins
// Part 1+2: 9 mins

use std::collections::HashSet;

fn run(title: &str, input: &str) {
    let (dots, folds) = input.split_once("\n\n").unwrap();

    let mut dots: Vec<(i32, i32)> = dots.lines().map(|n| {
        let (x, y) = n.split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }).collect();

    for fold in folds.lines() {
        let f = fold.strip_prefix("fold along ").unwrap();
        let axis = f.chars().nth(0).unwrap();
        let val: i32 = f[2..].parse().unwrap();

        if axis == 'x' {
            dots = dots.iter().map(|(mut x, y)| {
                if x > val {
                    x = val - (x - val);
                }
                (x, *y)
            }).collect();
        } else {
            dots = dots.iter().map(|(x, mut y)| {
                if y > val {
                    y = val - (y - val);
                }
                (*x, y)
            }).collect();
        }

        println!("{} part 1: {}", title, HashSet::<(i32, i32)>::from_iter(dots.iter().copied()).len());

        // break;
    }

    println!("{:?}", dots);

    for y in 0..10 {
        for x in 0..80 {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

const INPUT_DEMO: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("13/input.txt").unwrap());
}
