use std::{fs, collections::HashMap};

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct InputFile {
    dirs: Vec<Dir>,
    nodes: Vec<Node>,
}

peg::parser! {
    grammar input_parser() for str {
        rule left() -> Dir = "L" { Dir::L }
        rule right() -> Dir = "R" { Dir::R }

        pub rule dirs() -> Vec<Dir>
            = d:((left() / right())+) { d }

        rule ident() -> String
            = i:$(['0'..='9' | 'A'..='Z']+) { i.to_string() }

        pub rule node() -> Node
            = name:ident() " = (" left:ident() ", " right:ident() ")\n" { Node { name, left, right }}

        pub rule file() -> InputFile
            = dirs:dirs() "\n\n" nodes:(node()+) { InputFile { dirs, nodes } }
    }
}

fn part1(input: &InputFile) {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for node in &input.nodes {
        nodes.insert(node.name.clone(), (node.left.clone(), node.right.clone()));
    }

    let mut cur = "AAA".to_string();
    let mut steps = 0;
    while cur != "ZZZ" {
        let node = &nodes[&cur];
        cur = match input.dirs[steps % input.dirs.len()] {
            Dir::L => node.0.clone(),
            Dir::R => node.1.clone(),
        };
        steps += 1;
    }
    println!("Answer 1: {}", steps);
}

fn part2(input: &InputFile) {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for node in &input.nodes {
        nodes.insert(&node.name, (&node.left, &node.right));
    }

    let mut cur: Vec<_> = nodes.keys().filter(|n| n.ends_with("A")).collect();
    println!("{:?}", cur);
    let mut steps = 0;
    while !cur.iter().all(|n| n.ends_with("Z")) {
        cur = cur.iter().map(|c| {
            let node = &nodes[*c];
            match input.dirs[steps % input.dirs.len()] {
                Dir::L => &node.0,
                Dir::R => &node.1,
            }
        }).collect();
        steps += 1;
        // println!("{} {:?}", steps, cur);
    }
    println!("Answer 2: {}", steps);
}


/*
 * For each starting node:
 *  Count how many steps before reaching the next Z
 *  Continue until back at starting node *and* steps=0 (mod len)
 *  Then that node will loop
 *
 * Now take all starting nodes
 * Advance to the next
 *
 *
 * Find loops:
 *  For each (node,steps), find length of loop (or fail)
 *  + find indexes of ends points
 *
 * Iterate until every cur is in a loop
 */

/*
 * Or
 * Iterate 1M times, so we're all in loops at steps=0
 *
 * Examine each loop:
 */

// struct NodeData<'a> {
//     left: &'a str,
//     right: &'a str,
//     looplen: Option<usize>,
//     ends: Vec<usize>,
// }
#[derive(Debug)]
struct Loops {
    len: Option<usize>,
    ends: Vec<usize>,
}

fn part2b(input: &InputFile) {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for node in &input.nodes {
        nodes.insert(&node.name, (&node.left, &node.right));
    }
    // let mut nodes: HashMap<&str, NodeData> = HashMap::new();
    // for node in &input.nodes {
    //     nodes.insert(&node.name, NodeData { left: &node.left, right: &node.right, looplen: None, ends: Vec::new() });
    // }

    let mut loops = HashMap::new();

    // for start in nodes.keys().filter(|n| n.ends_with("A")) {
    for start in nodes.keys() {
            // for start_step in 0..input.dirs.len() {
        for start_step in 0..1 {
            let mut cur = start;
            let mut steps = 0;

            let mut curloop = Loops { len: None, ends: Vec::new() };

            for i in 0..(input.dirs.len() * nodes.len() + 1) {
                let node = &nodes[cur];
                cur = match input.dirs[steps % input.dirs.len()] {
                    Dir::L => &node.0,
                    Dir::R => &node.1,
                };
                steps += 1;
                if cur.ends_with("Z") {
                    curloop.ends.push(steps);
                }
                if cur == start && steps % input.dirs.len() == start_step {
                    curloop.len = Some(steps);
                    break;
                }
                // println!("{} {}", steps, cur);
                // if cur == start {
                //     break;
                // }
            }
            if curloop.len.is_some() {
                println!("{} {} {:?}", start, start_step, curloop);
                loops.insert((start, start_step), curloop);
            }
        }
    }

    // for start in nodes.keys().filter(|n| n.ends_with("A"))
    // {
    //     println!("=== {}", start);

    //     let mut ends: Vec<usize> = Vec::new();

    //     let mut cur = start;
    //     let mut steps = 0;
    //     // while !cur.ends_with("Z") {
    //     // loop {
    //     for i in 0..1000000 {
    //         let node = &nodes[cur];
    //         cur = match input.dirs[steps % input.dirs.len()] {
    //             Dir::L => &node.0,
    //             Dir::R => &node.1,
    //         };
    //         steps += 1;
    //         if cur.ends_with("Z") {
    //             ends.push(steps);
    //         }
    //         // println!("{} {}", steps, cur);
    //         // if cur == start {
    //         //     break;
    //         // }
    //     }
    //     println!("{:?}", ends.iter().map(|n| n % (nodes.len() * input.dirs.len())).collect::<Vec<_>>());
    //     // println!("{} {:?}", steps, ends);
    // }


    // let mut cur: Vec<_> = nodes.keys().filter(|n| n.ends_with("A")).collect();
    // println!("{:?}", cur);
    // let mut steps = 0;
    // while !cur.iter().all(|n| n.ends_with("Z")) {
    //     cur = cur.iter().map(|c| {
    //         let node = &nodes[*c];
    //         match input.dirs[steps % input.dirs.len()] {
    //             Dir::L => &node.0,
    //             Dir::R => &node.1,
    //         }
    //     }).collect();
    //     steps += 1;
    //     // println!("{} {:?}", steps, cur);
    // }
    // println!("Answer 2: {}", steps);
}

pub fn lcm(nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

pub fn gcd(nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = gcd(&nums[1..]);
    gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn part2c(input: &InputFile) {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for node in &input.nodes {
        nodes.insert(&node.name, (&node.left, &node.right));
    }

    let start_steps;
    let mut cur: Vec<_> = nodes.keys().filter(|n| n.ends_with("A")).collect();
    {
        println!("{:?}", cur);
        let mut steps = 0;
        for i in 0..(input.dirs.len() * nodes.len()) {
            cur = cur.iter().map(|c| {
                let node = &nodes[*c];
                match input.dirs[steps % input.dirs.len()] {
                    Dir::L => &node.0,
                    Dir::R => &node.1,
                }
            }).collect();
            if cur.iter().all(|n| n.ends_with("Z")) {
                panic!();
            }
            steps += 1;
            // println!("{} {:?}", steps, cur);
        }
        start_steps = steps;
    }
    println!("{:?}", cur);

    let starts = cur;

    let mut loops = Vec::new();
    for start in starts {
        println!("{}", start);
        let mut cur = start;
        let mut steps = 0;
        let mut ends = Vec::new();
        for i in 0..1000000 {
            let node = &nodes[cur];
            cur = match input.dirs[steps % input.dirs.len()] {
                Dir::L => &node.0,
                Dir::R => &node.1,
            };
            steps += 1;
            if cur.ends_with("Z") {
                ends.push(steps);
            }
            if cur == start && steps % input.dirs.len() == 0 {
                println!("Loop {} {:?}", steps, ends);
                loops.push((steps, ends[0]));
                break;
            }
        }
    }

    println!("{}, {:?}", start_steps, loops);

    let ss: Vec<_> = loops.iter().map(|(s, e)| *s as i128).collect();
    let es: Vec<_> = loops.iter().map(|(s, e)| *e as i128).collect();
    let gcd = gcd(&ss);
    let ss: Vec<_> = loops.iter().map(|(s, e)| *s as i128 / gcd).collect();
    let es: Vec<_> = loops.iter().map(|(s, e)| *e as i128 / gcd).collect();
    // println!("s={:?} e={:?} gcd={}", ss, es, gcd(&ss));
    let answer = chinese_remainder(&es, &ss).unwrap() * gcd;

    // Answer must be
    //   x % 13301 = 10754
    //   x % 22357 = 21508
    //   x % 17263 = 5094
    //   ...

    println!("Answer 2: {} {}", answer, start_steps as i128 + answer);
}

fn main() {
    let input = input_parser::file(&fs::read_to_string("input").unwrap()).unwrap();
    // println!("{:#?}", input);
    // part1(&input);
    part2c(&input);
}
