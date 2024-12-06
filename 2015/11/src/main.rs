use core::str;

use itertools::Itertools;

fn next(mut p: [u8; 8]) -> [u8; 8] {
    p[7] += 1;
    for i in (0..8).rev() {
        if p[i] > b'z' {
            p[i] = b'a';
            p[i - 1] += 1;
        }
    }
    p
}

fn ok(p: [u8; 8]) -> bool {
    if p.contains(&b'i') || p.contains(&b'o') || p.contains(&b'l') {
        return false;
    }

    if !p.iter().tuple_windows().any(|(&a, &b, &c)| b == a+1 && c == b+1) {
        return false;
    }

    let mut overlap = false;
    let mut doubles = 0;
    for (&a, &b) in p.iter().tuple_windows() {
        if !overlap && a == b {
            doubles += 1;
            overlap = true;
        } else {
            overlap = false;
        }
    }

    if doubles != 2 {
        return false;
    }

    true
}

fn run(title: &str, input: &str) {
    let mut cs: [u8; 8] = input.as_bytes().try_into().unwrap();
    while !ok(cs) {
        cs = next(cs);
    }
    println!("{} part 1: {}", title, str::from_utf8(&cs).unwrap());

    cs = next(cs);
    while !ok(cs) {
        cs = next(cs);
    }
    println!("{} part 2: {}", title, str::from_utf8(&cs).unwrap());
}

const INPUT_DEMO: &str = "";

fn main() {
    run("demo 1", "abcdefgh");
    run("demo 2", "ghjaabcc");
    run("input", "hepxcrrq");
}
