// Part 1: 5 mins
// Part 1+2: 7 mins

fn value(x: i32, y: i32, input: i32) -> i32 {
    let rack = x + 10;
    let power = (rack * y + input) * rack;
    let power = (power / 100) % 10;
    power - 5
}

fn run(title: &str, input: i32) {
    let mut best = (-i32::MAX, 0, 0);
    for y in 1..300 {
        for x in 1..300 {
            let mut sum = 0;
            for dy in 0..3 {
                for dx in 0..3 {
                    sum += value(x + dx, y + dy, input);
                }
            }
            best = best.max((sum, x, y));
        }
    }

    println!("{} part 1: {:?}", title, best);

    let mut best = (-i32::MAX, 0, 0, 0);
    for s in 1..=300 {
        println!("{} {:?}", s, best);
        for y in 1..=300-s {
            for x in 1..=300-s {
                let mut sum = 0;
                for dy in 0..s {
                    for dx in 0..s {
                        sum += value(x + dx, y + dy, input);
                    }
                }
                best = best.max((sum, x, y, s));
            }
        }
    }

    println!("{} part 2: {:?}", title, best);
}

fn main() {
    // run("demo", 18);
    run("input", 5177);
}
