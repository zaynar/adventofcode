// Part 1: 12 mins
// Part 1+2: 13 mins

use std::collections::HashMap;

fn run(title: &str, input: &str) {

    let mut path = vec![];

    let mut total_size: HashMap<Vec<String>, u32> = HashMap::new();

    for cmd in input.split("$ ").skip(1) {
        let mut lines = cmd.lines();
        let cmd = lines.next().unwrap();
        if cmd == "cd /" {
            path.clear();
        } else if cmd == "cd .." {
            path.pop();
        } else if cmd.starts_with("cd ") {
            path.push(cmd[3..].to_owned());
        } else if cmd == "ls" {
            let size: u32 = lines.filter_map(|e| e.split_once(" ").unwrap().0.parse::<u32>().ok()).sum();
            for i in 0..=path.len() {
                *total_size.entry(path[0..i].to_vec()).or_insert(0) += size;
            }
        } else {
            panic!();
        }
    }

    // println!("{:?}", total_size);

    println!("{} part 1: {}", title, total_size.values().map(|n| if *n <= 100000 { *n } else { 0 }).sum::<u32>());

    let target = total_size[&vec![]] - 40000000;

    println!("{} part 2: {}", title, total_size.values().filter(|n| **n >= target).min().unwrap());
}

const INPUT_DEMO: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
