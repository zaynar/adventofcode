// Part 1: 13 mins
// Part 1+2: 14 mins

#[derive(Clone)]
struct Node {
    prev: usize,
    next: usize,
    val: u32,
}

fn print(circle: &Vec<Node>, start: usize) {
    let mut node = start;
    loop {
        print!("{:2} ", circle[node].val);
        node = circle[node].next;
        if node == start {
            break;
        }
    }
    println!();
}

fn run(title: &str, players: usize, last_marble: u32) {
    let mut scores = vec![0; players];
    let mut circle = vec![Node { prev: 0, next: 0, val: 0 }];
    let mut current = 0;

    for marble in 1..=last_marble {
        // print(&circle, current);

        if marble % 23 == 0 {
            for i in 0..7 {
                current = circle[current].prev;
            }
            scores[marble as usize % players] += marble;
            scores[marble as usize % players] += circle[current].val;
            let c = circle[current].clone();
            circle[c.prev].next = c.next;
            circle[c.next].prev = c.prev;
            current = c.next;

        } else {
            current = circle[current].next;

            let next = circle[current].next;
            let new = circle.len();
            circle.push(Node { prev: current, next, val: marble });
            circle[next].prev = new;
            circle[current].next = new;
            current = new;
        }
    }

    println!("{} part 1: {}", title, scores.iter().max().unwrap());
}

fn main() {
    run("demo", 9, 25);
    run("demo", 19, 1618);
    run("demo", 30, 5807);
    run("input", 427, 70723);
    run("input", 427, 70723 * 100);
}
