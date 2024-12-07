use std::collections::VecDeque;

fn run(title: &str, n: usize) {
    let mut elfs = vec![1; n];

    let mut i = 0;
    loop {
        // println!("{:?}", elfs);
        let mut j = i + 1;
        while elfs[j % n] == 0 {
            j += 1;
        }
        if (j - i) % n == 0 {
            println!("{} part 1: {}", title, i % n + 1);
            break;
        }
        elfs[i % n] += elfs[j % n];
        elfs[j % n] = 0;

        i += 1;
        while elfs[i % n] == 0 {
            i += 1;
        }
    }


    // println!("{} part 2: {}", title, "TODO");
}

fn run2(title: &str, input: usize) {

    let mut elfs = VecDeque::new();
    for i in 0..input {
        elfs.push_back(i as u32 + 1);
    }

    while elfs.len() > 1 {
        let n = elfs.len();

        if n % 1000 == 0 {
            println!("{}", n);
        }

        elfs.remove(n / 2);

        let top = elfs.pop_front().unwrap();
        elfs.push_back(top);
    }

    println!("{} part 2: {:?}", title, elfs);
}

struct Node {
    id: u32,
    prev: usize,
    next: usize,
}

fn run3(title: &str, n: usize) {
    let mut elfs = Vec::new();
    for i in 0..n {
        elfs.push(Node { id: i as u32 + 1, prev: (i + n - 1) % n, next: (i + 1) % n });
    }

    let mut cur = 0;
    let mut target = n / 2;
    let mut dist = target - cur;
    let mut rem = n;

    loop {
        // Remove target from list

        let tp = elfs[target].prev;
        let tn = elfs[target].next;
        elfs[tp].next = elfs[target].next;
        elfs[tn].prev = elfs[target].prev;
        target = elfs[target].prev;
        dist -= 1;
        rem -= 1;

        while dist < rem / 2 {
            target = elfs[target].next;
            dist += 1;
        }

        cur = elfs[cur].next;
        target = elfs[target].next;
        if target == cur {
            println!("{} part 2: {:?}", title, elfs[cur].id);
            break;
        }
    }
}


fn main() {
    // run("demo", 5);
    // run("input", 3004953);
    // run2("demo", 5);
    // run2("input", 3004953);
    run3("demo", 5);
    run3("input", 3004953);
}
