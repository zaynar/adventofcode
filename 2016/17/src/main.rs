use std::collections::VecDeque;

use md5::{Md5, Digest};

#[derive(Debug)]
struct Node {
    x: u32,
    y: u32,
    path: String,
}

fn run(title: &str, input: &str) {
    let mut hasher = Md5::new();
    hasher.update(input);

    let mut open = VecDeque::new();
    open.push_back(Node { x: 0, y: 0, path: "".to_owned() });

    let mut longest = 0;

    while let Some(n) = open.pop_front() {

        if (n.x, n.y) == (3, 3) {
            // Part 1:
            // println!("{:?}", n);
            // break;

            longest = longest.max(n.path.len());
            continue;
        }

        let mut h = hasher.clone();
        h.update(&n.path);
        let hash = hex::encode(h.finalize()).into_bytes();

        if n.y > 0 && b"bcdef".contains(&hash[0]) {
            open.push_back(Node { x: n.x, y: n.y - 1, path: n.path.clone() + "U" });
        }
        if n.y < 3 && b"bcdef".contains(&hash[1]) {
            open.push_back(Node { x: n.x, y: n.y + 1, path: n.path.clone() + "D" });
        }
        if n.x > 0 && b"bcdef".contains(&hash[2]) {
            open.push_back(Node { x: n.x - 1, y: n.y, path: n.path.clone() + "L" });
        }
        if n.x < 3 && b"bcdef".contains(&hash[3]) {
            open.push_back(Node { x: n.x + 1, y: n.y, path: n.path.clone() + "R" });
        }

    }

    println!("{} part 2: {}", title, longest);

}

fn main() {
    run("demo", "ulqzkmiv");
    run("input", "vwbaicqe");
}
