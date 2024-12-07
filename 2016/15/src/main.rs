fn run(title: &str, discs: Vec<(u64, u64)>) {

    for p in 0.. {
        if discs.iter().enumerate().all(|(i, d)| {
            ((p + i as u64 + 1) + d.1) % d.0 == 0
        }) {
            println!("{}: {}", title, p);
            return;
        }
    }
}

fn main() {
/*
    Disc #1 has 5 positions; at time=0, it is at position 2.
    Disc #2 has 13 positions; at time=0, it is at position 7.
    Disc #3 has 17 positions; at time=0, it is at position 10.
    Disc #4 has 3 positions; at time=0, it is at position 2.
    Disc #5 has 19 positions; at time=0, it is at position 9.
    Disc #6 has 7 positions; at time=0, it is at position 0.
*/

    run("input", vec![(5, 4), (2, 1)]);

    let discs = vec![
        (5, 2),
        (13, 7),
        (17, 10),
        (3, 2),
        (19, 9),
        (7, 0),
    ];

    run("input", discs);

    let discs = vec![
        (5, 2),
        (13, 7),
        (17, 10),
        (3, 2),
        (19, 9),
        (7, 0),
        (11, 0),
    ];

    run("input", discs);
}
