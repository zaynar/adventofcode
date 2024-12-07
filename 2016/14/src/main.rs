use itertools::Itertools;
use md5::{Md5, Digest};

fn has_run(input: &str, len: usize) -> Option<u8> {
    for w in input.as_bytes().windows(len) {
        if w.iter().all(|c| *c == w[0]) {
            return Some(w[0]);
        }
    }
    None
}

fn run(title: &str, input: &str) {
    let mut hasher = Md5::new();
    hasher.update(input);

    // let hashes = (0..30_000).map(|i| {
    //     let mut h = hasher.clone();
    //     h.update(i.to_string().as_bytes());
    //     hex::encode(h.finalize())
    // }).collect_vec();

    let hashes = (0..30_000).map(|i| {
        let mut h = hasher.clone();
        h.update(i.to_string().as_bytes());
        let mut s = hex::encode(h.finalize());

        for _ in 0..2016 {
            let mut h = Md5::new();
            h.update(s);
            s = hex::encode(h.finalize());
        }
        s
    }).collect_vec();

    let mut n = 0;
    'OUTER: for i in 0.. {
        if let Some(c) = has_run(&hashes[i], 3) {
            let pat: String = [c as char; 5].iter().collect();
            for j in 1..=1000 {
                if hashes[i+j].contains(&pat) {
                    n += 1;
                    if n == 64 {
                        println!("{}", i);
                        break 'OUTER;
                    }

                    continue 'OUTER;
                }
            }
        }
    }


}

fn main() {
    run("demo", "abc");
    run("input", "ahsbgdzn");
}
