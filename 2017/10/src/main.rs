// Part 1: 6 mins
// Part 1+2: 13 mins

use itertools::Itertools;

fn run(title: &str, input: &str, len: usize) {
    let data: Vec<usize> = input.trim().split(",").map(|n| n.parse().unwrap()).collect_vec();

    // println!("{:?}", data);

    let mut a = (0..len).collect_vec();

    let mut curr = 0;
    let mut ss = 0;
    for step in &data {
        // println!("{:?}", a);

        let mut b = a.clone();
        for i in 0..*step {
            b[(curr + i) % len] = a[(curr + len + step-1 - i) % len];
        }
        curr += step + ss;
        ss += 1;
        a = b;

        // println!("{:?}\n", a);
    }

    println!("{} part 1: {}", title, a[0] * a[1]);
}

fn run2(title: &str, input: &str) {
    let mut data: Vec<usize> = input.trim().chars().map(|c| c as u8 as usize).collect_vec();
    data.append(&mut vec![17, 31, 73, 47, 23]);

    println!("{:?}", data);

    let len = 256;
    let mut curr = 0;
    let mut ss = 0;

    let mut a = (0..len).collect_vec();

    for round in 0..64 {

        for step in &data {
            // println!("{:?}", a);

            let mut b = a.clone();
            for i in 0..*step {
                b[(curr + i) % len] = a[(curr + len + step-1 - i) % len];
            }
            curr += step + ss;
            ss += 1;
            a = b;

            // println!("{:?}\n", a);
        }
    }

    let hash = (0..16).map(|i| {
        let mut n = 0;
        for j in 0..16 {
            n ^= a[i*16 + j];
        }
        format!("{:02x}", n)
    }).collect::<String>();

    println!("{} part 2: {}", title, hash);
}


fn main() {
    run("demo", "3,4,1,5", 5);
    run("input", &std::fs::read_to_string("10/input.txt").unwrap(), 256);

    run2("demo", "");
    run2("demo", "AoC 2017");
    run2("input", &std::fs::read_to_string("10/input.txt").unwrap());
}
