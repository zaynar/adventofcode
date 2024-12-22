// Part 1: 15 mins
// Part 1+2: 25 mins

use itertools::Itertools;

#[derive(Clone)]
struct Node {
    // prev: usize,
    next: usize,
    val: u32,
}

fn print(circle: &Vec<Node>, start: usize) {
    let mut node = start;
    loop {
        print!("{:2} ", circle[node].val);
        // assert_eq!(circle[circle[node].next].prev, node);
        node = circle[node].next;
        if node == start {
            break;
        }
    }
    println!();
}

fn printn(circle: &Vec<Node>, start: usize, max: usize) {
    let mut node = start;
    for _ in 0..max {
        print!("{:2} ", circle[node].val);
        // assert_eq!(circle[circle[node].next].prev, node);
        node = circle[node].next;
        if node == start {
            break;
        }
    }
    println!();
}

fn run(title: &str, input: u32) {
    let mut cups = vec![];

    let mut index = vec![0; 10];
    let input = input.to_string().chars().map(|n| n.to_digit(10).unwrap()).collect_vec();
    let len = input.len();
    for i in 0..len {
        cups.push(Node {
            // prev: (i + len - 1) % len,
            next: (i + 1) % len,
            val: input[i]
        });
        index[input[i] as usize] = i;
    }

    let mut current = 0;

    print(&cups, current);

    for step in 0..100 {
        let a = cups[current].next;
        let b = cups[a].next;
        let c = cups[b].next;
        let d = cups[c].next;
        cups[current].next = d;
        // cups[d].prev = current;
        // print(&cups, current);

        let mut dest = cups[current].val - 1;
        if dest == 0 { dest = 9; }
        while [cups[a].val, cups[b].val, cups[c].val].contains(&dest) {
            dest -= 1;
            if dest == 0 { dest = 9; }
        }

        // println!("dest {}", dest);

        let di = index[dest as usize];
        cups[c].next = cups[di].next;
        cups[di].next = a;
        // XXX prev

        // print(&cups, current);
        current = cups[current].next;

        // break;
    }

    print(&cups, current);

    current = index[1];
    let mut part1 = "".to_owned();
    for i in 0..8 {
        current = cups[current].next;
        part1 += &cups[current].val.to_string();
    }

    println!("{} part 1: {}", title, part1);
}

fn run2(title: &str, input: u32) {
    let mut cups = vec![];

    let len = 1_000_000;
    let reps = 10_000_000;

    let input = input.to_string().chars().map(|n| n.to_digit(10).unwrap()).collect_vec();
    let mut index = vec![0; 1 + len];

    for i in 0..len {
        let val = input.get(i).copied().unwrap_or(i as u32 + 1);
        cups.push(Node {
            // prev: (i + len - 1) % len,
            next: (i + 1) % len,
            val
        });
        index[val as usize] = i;
    }

    let mut current = 0;
    // print(&cups, current);

    for step in 0..reps {
        let a = cups[current].next;
        let b = cups[a].next;
        let c = cups[b].next;
        let d = cups[c].next;
        cups[current].next = d;
        // cups[d].prev = current;
        // print(&cups, current);

        let mut dest = cups[current].val - 1;
        if dest == 0 { dest = len as u32; }
        while [cups[a].val, cups[b].val, cups[c].val].contains(&dest) {
            dest -= 1;
            if dest == 0 { dest = len as u32; }
        }

        // println!("dest {}", dest);

        let di = index[dest as usize];
        cups[c].next = cups[di].next;
        cups[di].next = a;

        // print(&cups, current);
        current = cups[current].next;

        // break;
    }

    // print(&cups, current);

    printn(&cups, index[1], 10);

    current = index[1];
    current = cups[current].next;
    let v0 = cups[current].val as u64;
    current = cups[current].next;
    let v1 = cups[current].val as u64;

    println!("{} part 2: {}*{} = {}", title, v0, v1, v0 * v1);
}

fn main() {
    run("demo", 389125467);
    run("input", 394618527);
    run2("demo", 389125467);
    run2("input", 394618527);
}

