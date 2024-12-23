// Part 1: 11 mins
// Part 1+2: 12 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (nums, boards) = input.split_once("\n\n").unwrap();
    let nums: Vec<u32> = nums.split(",").map(|n| n.parse().unwrap()).collect_vec();
    let boards: Vec<Vec<u32>> = boards.split("\n\n").map(|b|
        b.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect_vec()
    ).collect_vec();

    'OUTER: for i in 0..nums.len() {
        for b in boards.iter() {
            let marked = b.iter().map(|n| nums[0..i].contains(n)).collect_vec();

            let mut won = false;
            for j in 0..5 {
                if (0..5).all(|k| marked[k + j*5]) ||
                   (0..5).all(|k| marked[j + k*5])
                //    (0..5).all(|k| marked[k + k*5]) ||
                //    (0..5).all(|k| marked[(4-k) + k*5])
                    {
                    won = true;
                }
            }

            // println!("won {} {} {:?} {:?}", i, nums[i], b, marked);
            if won {
                let part1 = nums[i-1] * marked.iter().enumerate().filter_map(|(i, m)| if !*m { Some(b[i]) } else { None }).sum::<u32>();
                println!("{} part 1: {}", title, part1);
                break 'OUTER;
            }
        }
    }

    println!("{} part 1: {}", title, "TODO");

    let mut bwon = vec![false; boards.len()];
    'OUTER: for i in 0..nums.len() {
        for (bi, b) in boards.iter().enumerate() {
            if bwon[bi] { continue; }

            let marked = b.iter().map(|n| nums[0..i].contains(n)).collect_vec();

            let mut won = false;
            for j in 0..5 {
                if (0..5).all(|k| marked[k + j*5]) ||
                   (0..5).all(|k| marked[j + k*5])
                //    (0..5).all(|k| marked[k + k*5]) ||
                //    (0..5).all(|k| marked[(4-k) + k*5])
                    {
                    won = true;
                }
            }

            // println!("won {} {} {:?} {:?}", i, nums[i], b, marked);
            if won {
                bwon[bi] = true;
                if bwon.iter().all(|n| *n) {
                    let part1 = nums[i-1] * marked.iter().enumerate().filter_map(|(i, m)| if !*m { Some(b[i]) } else { None }).sum::<u32>();
                    println!("{} part 2: {}", title, part1);
                    break 'OUTER;
                }
            }
        }
    }
}

const INPUT_DEMO: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
