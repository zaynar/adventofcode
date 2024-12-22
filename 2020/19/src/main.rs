// Part 1: 21 mins
// Part 1+2: 29 mins

use std::{collections::HashMap};

use itertools::Itertools;

#[derive(Debug)]
enum Rule {
    Char(char),
    Seq(Vec<u32>),
    Alt(Vec<u32>, Vec<u32>),
}

fn matseq(rules: &HashMap<u32, Rule>, vec: &Vec<u32>, s: &[char]) -> Vec<usize> {
    let mut offsets = vec![0];
    for v in vec {
        let mut newoffsets = vec![];
        for o in &offsets {
            for z in mat(rules, *v, &s[*o..]) {
                newoffsets.push(o + z);
            }
        }
        offsets = newoffsets;
        offsets.sort();
        offsets.dedup();
    }
    return offsets;
}

fn mat(rules: &HashMap<u32, Rule>, id: u32, s: &[char]) -> Vec<usize> {
    match rules.get(&id).unwrap() {
        Rule::Char(c) => {
            if s.len() > 0 && s[0] == *c {
                return vec![1];
            } else {
                return vec![];
            }
        },
        Rule::Seq(vec) => {
            return matseq(rules, vec, s);
        }
        Rule::Alt(v0, v1) => {
            let mut r = matseq(rules, v0, s);
            r.append(&mut matseq(rules, v1, s));
            r.sort();
            r.dedup();
            return r;
        }
    }
}

fn run(title: &str, input: &str) {
    let mut rules = HashMap::new();

    let (r, msgs) = input.split_once("\n\n").unwrap();

    for line in r.lines() {
        let (id, val) = line.split_once(": ").unwrap();
        let id: u32 = id.parse().unwrap();
        let rule = if val.starts_with("\"") {
            Rule::Char(val.chars().nth(1).unwrap())
        } else if val.contains("|") {
            let (a, b) = val.split_once(" | ").unwrap();
            let a = a.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect_vec();
            let b = b.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect_vec();
            Rule::Alt(a, b)
        } else {
            let a = val.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect_vec();
            Rule::Seq(a)
        };

        rules.insert(id, rule);
    }

    let mut part1 = 0;
    for msg in msgs.lines() {

        if mat(&rules, 0, &msg.chars().collect_vec()).iter().any(|n| *n == msg.len()) {
            // println!("{}", msg);
            part1 += 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    rules.insert(8, Rule::Alt(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Alt(vec![42, 31], vec![42, 11, 31]));

    let mut part2 = 0;
    for msg in msgs.lines() {
        if mat(&rules, 0, &msg.chars().collect_vec()).iter().any(|n| *n == msg.len()) {
            // println!("{}", msg);
            part2 += 1;
        }
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

const INPUT_DEMO2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

fn main() {
    run("demo", INPUT_DEMO);
    run("demo2", INPUT_DEMO2);
    run("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
