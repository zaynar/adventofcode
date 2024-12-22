// Part 1: 13 mins
// Part 1+2: 13 mins

use std::collections::HashMap;

fn muladd(state: &HashMap<i128, (i128, i128)>, p: i128, m: i128, reps: i64) -> i128 {
    if reps == 0 {
        return p;
    }

    let mut pow2 = 0;
    while (1 << (pow2 + 1)) < reps {
        pow2 += 1;
    }

    // println!("{} = 1<<{} + {}", reps, pow2, reps - (1 << pow2));
    let (pa, pb) = state.get(&pow2).unwrap();
    let q = modulo(p * pa + pb, m);

    muladd(state, q, m, reps - (1 << pow2))
}

fn modulo(a: i128, m: i128) -> i128 {
    (a + m) % m
}

fn transform(subject: i128, loopsize: usize) -> i128 {
    let mut n = 1;
    for i in 0..loopsize {
        n = (n * subject) % 20201227;
    }
    n
}

fn transforms(state: &mut HashMap<i128, (i128, i128)>, loopsize: usize) -> i128 {

    let m = 20201227;

    muladd(state, 1, m, loopsize as i64)
}

fn run(title: &str, card_pub: i128, door_pub: i128) {

    let mut card_loop = 0;
    let mut door_loop = 0;

    let m = 20201227;
    let subject = 7;
    let mut state = HashMap::new();
    state.insert(0, (subject, 0));
    for i in 1..64 {
        let (a, b) = state[&(i-1)];
        state.insert(i, (modulo(a * a, m), modulo(b * a + b, m)));
    }

    for i in 0.. {
        if transforms(&mut state, i) == card_pub {
            card_loop = i;
            break;
        }
    }
    println!("card_loop={}", card_loop);

    for i in 0.. {
        if transforms(&mut state, i) == door_pub {
            door_loop = i;
            break;
        }
    }
    println!("door_loop={}", door_loop);

    let key1 = transform(door_pub, card_loop);
    let key2 = transform(card_pub, door_loop);

    println!("{} part 1: {} {}", title, key1, key2);
}

fn main() {
    run("demo", 5764801, 17807724);
    run("input", 15733400, 6408062);
}