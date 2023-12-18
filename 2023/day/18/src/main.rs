use std::{fs, collections::HashMap};

fn main() {
    let input: Vec<_> = fs::read_to_string("input").unwrap().lines().map(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let dir = tokens.next().unwrap().to_owned();
        let dist = tokens.next().unwrap().parse::<i32>().unwrap();
        let colour = tokens.next().unwrap()[2..8].to_owned();
        (dir, dist, colour)
    }).collect();
    // println!("{:?}", input);

    let mut minpos = (0, 0);
    let mut maxpos = (0, 0);
    let mut trench: HashMap<(i32, i32), bool> = HashMap::new();
    let mut pos = (0, 0);
    for (dir, dist, colour) in input {
        let (dx, dy) = match dir.as_str() {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        for i in 0..dist {
            trench.insert(pos, true);
            pos = (pos.0 + dx, pos.1 + dy);
        }

        minpos = (minpos.0.min(pos.0), minpos.1.min(pos.1));
        maxpos = (maxpos.0.max(pos.0), maxpos.1.max(pos.1));
    }

    println!("{:?} {:?}", minpos, maxpos);

    let mut dug = 0;
    for y in minpos.1..=maxpos.1 {
        let mut inside = false;
        for x in minpos.0..=maxpos.0 {
            // print!("{}", if *trench.get(&(x, y)).unwrap_or(&false) { "#" } else { "." });

            if *trench.get(&(x, y)).unwrap_or(&false) {
                print!("#");
                dug += 1;
                if *trench.get(&(x, y+1)).unwrap_or(&false) {
                    inside = !inside;
                }
              } else if inside {
                print!("@");
                dug += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("Answer 1: {}", dug);
}
