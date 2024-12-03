use itertools::Itertools;

fn run(title: &str, input: &str) {
    let data = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect_vec();

    let w = 25;
    let h = 6;

    let mut cs = Vec::new();
    for z in 0..(data.len() / (w*h)) {
        let mut counts = vec![0, 0, 0];
        for y in 0..h {
            for x in 0..w {
                counts[data[x + y*w + z*w*h] as usize] += 1;
            }
        }
        cs.push(counts);
    }

    cs.sort_by_key(|c| c[0]);

    println!("{} part 1: {}", title, cs[0][1] * cs[0][2]);

    for y in 0..h {
        for x in 0..w {
            let mut p = 9;
            for z in (0..(data.len() / (w*h))).rev() {
                let o = data[x + y*w + z*w*h];
                if o != 2 {
                    p = o;
                }
            }
            print!("{}", if p == 1 { "#" } else {"." } );
        }
        println!();
    }

    println!("{} part 2: {}", title, "TODO");
}

fn main() {
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
