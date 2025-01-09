// Part 1: 11 mins
// Part 1+2: 15 mins

use std::cmp::Ordering;

use serde_json::Value::{Number, Array};

fn compare(a: &serde_json::Value, b: &serde_json::Value) -> Ordering {
    match (a, b) {
        (Number(a), Number(b)) => a.as_u64().cmp(&b.as_u64()),
        (Array(a), Array(b)) => {
            for (a, b) in a.iter().zip(b.iter()) {
                let c = compare(a, b);
                if c != Ordering::Equal {
                    return c;
                }
            }
            return a.len().cmp(&b.len());
        }
        (Array(a), Number(b)) => {
            compare(&Array(a.clone()), &Array(vec![Number(b.clone())]))
        }
        (Number(a), Array(b)) => {
            compare(&Array(vec![Number(a.clone())]), &Array(b.clone()))
        }
        _ => panic!(),
    }
}



fn run(title: &str, input: &str) {

    let mut part1 = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let (a, b) = pair.trim().split_once("\n").unwrap();

        let a: serde_json::Value = serde_json::from_str(a).unwrap();
        let b: serde_json::Value = serde_json::from_str(b).unwrap();

        // println!("{:?} {:?}", compare(&a, &b), (a, b));
        if compare(&a, &b) == Ordering::Less {
            part1 += i + 1;
        }
    }

    println!("{} part 1: {}", title, part1);

    let div1 = Array(vec![Array(vec![Number(serde_json::Number::from_u128(2).unwrap())])]);
    let div2 = Array(vec![Array(vec![Number(serde_json::Number::from_u128(6).unwrap())])]);
    let mut packets = vec![div1.clone(), div2.clone()];
    for (i, pair) in input.split("\n\n").enumerate() {
        let (a, b) = pair.trim().split_once("\n").unwrap();

        packets.push(serde_json::from_str(a).unwrap());
        packets.push(serde_json::from_str(b).unwrap());
    }

    packets.sort_by(compare);

    // println!("{:?}", packets);

    println!("{} part 2: {}", title, (packets.iter().position(|p| *p == div1).unwrap() + 1) * (packets.iter().position(|p| *p == div2).unwrap() + 1));
}

const INPUT_DEMO: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("13/input.txt").unwrap());
}
