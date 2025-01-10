// Part 1: 19 mins
// Part 1+2: 21 mins

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Node {
    n: i64,
    prev: usize,
    next: usize,
}

fn run(title: &str, input: &str) {
    let data: Vec<i64> = input
        .lines()
        .map(|n| n.parse().unwrap()).collect();

    let len = data.len();
    let mut list = data.iter().enumerate().map(|(i, n)| Node { n: *n, prev: (i + len - 1) % len, next: (i + 1) % len}).collect_vec();

    // println!("{:?}", list);

    for i in 0..len {
        let mut node = list[i].clone();

        // println!("moving {}", node.n);

        if node.n == 0 {
            continue;
        }

        list[node.prev].next = node.next;
        list[node.next].prev = node.prev;

        let mut t = i;
        if node.n >= 0 {
            for j in 0..node.n {
                t = list[t].next;
            }
        } else {
            for j in 0..(1-node.n) {
                t = list[t].prev;
            }
        }

        let s = list[t].next;
        node.prev = t;
        node.next = s;
        list[s].prev = i;
        list[t].next = i;
        list[i] = node;


        // println!("{:?}", list);
        // let mut t = 0;
        // for j in 0..len {
        //     print!("{} ", list[t].n);
        //     assert_eq!(list[list[t].next].prev, t);
        //     t = list[t].next;
        // }
        // println!();
    }

    let mut part1 = 0;
    {
        let mut t = list.iter().position(|p| p.n == 0).unwrap();
        for j in 0..=3000 {
            if [1000, 2000, 3000].contains(&j) {
            // if j == 0 && 990 < j && j < 1010 {
                // println!("{} is {}", j, list[t].n);
                part1 += list[t].n;
            }
            assert_eq!(list[list[t].next].prev, t);
            t = list[t].next;
        }
    }
    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, "TODO");
}

fn run2(title: &str, input: &str) {
    let data: Vec<i64> = input
        .lines()
        .map(|n| n.parse().unwrap()).collect();

    let len = data.len();
    let mut list = data.iter().enumerate().map(|(i, n)| Node { n: *n * 811589153, prev: (i + len - 1) % len, next: (i + 1) % len}).collect_vec();

    // println!("{:?}", list);

    for k in 0..10 {
        for i in 0..len {
            let mut node = list[i].clone();

            // println!("moving {}", node.n);

            if node.n == 0 {
                continue;
            }

            list[node.prev].next = node.next;
            list[node.next].prev = node.prev;

            let mut t = i;
            if node.n >= 0 {
                for j in 0 .. node.n % (len as i64 - 1) {
                    t = list[t].next;
                }
            } else {
                for j in 0 .. (1-node.n) % (len as i64 - 1) {
                    t = list[t].prev;
                }
            }

            let s = list[t].next;
            node.prev = t;
            node.next = s;
            list[s].prev = i;
            list[t].next = i;
            list[i] = node;


            // println!("{:?}", list);
            // let mut t = 0;
            // for j in 0..len {
            //     print!("{} ", list[t].n);
            //     assert_eq!(list[list[t].next].prev, t);
            //     t = list[t].next;
            // }
            // println!();
        }
    }

    let mut part2 = 0;
    {
        let mut t = list.iter().position(|p| p.n == 0).unwrap();
        for j in 0..=3000 {
            if [1000, 2000, 3000].contains(&j) {
            // if j == 0 && 990 < j && j < 1010 {
                // println!("{} is {}", j, list[t].n);
                part2 += list[t].n;
            }
            assert_eq!(list[list[t].next].prev, t);
            t = list[t].next;
        }
    }
    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "1
2
-3
3
-2
0
4
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("20/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("20/input.txt").unwrap());
}
