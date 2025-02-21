use std::fs;

fn hash(s: &str) -> usize {
    let mut val = 0;
    for c in s.chars() {
        val += c as usize;
        val = (val * 17) % 256;
    }
    val
}

fn update(b: &mut Vec<(String, usize)>, lens: &str, focus: usize) {
    for c in b.iter_mut() {
        if c.0 == lens {
            c.1 = focus;
            return;
        }
    }
    b.push((lens.to_string(), focus));
}

fn main() {
    let mut boxes: Vec<Vec<(String, usize)>> = (0..256).into_iter().map(|_| Vec::new()).collect();
    let mut sum = 0;
    for cmd in fs::read_to_string("input").unwrap().trim_end().split(",") {
        // println!("{} {}", cmd, hash(cmd));
        sum += hash(cmd);

        if cmd.ends_with("-") {
            let lens = &cmd[0..cmd.len()-1];
            // println!("{}: delete {}", cmd, lens);
            let b = &mut boxes[hash(lens)];
            b.retain(|c| c.0 != lens);
        } else {
            let lens = &cmd[0..cmd.len()-2];
            let focus = cmd.chars().last().unwrap().to_digit(10).unwrap() as usize;
            // println!("{}: set {} {}", cmd, lens, focus);
            let b = &mut boxes[hash(lens)];
            update(b, lens, focus);
        }
        // println!("{:?}", boxes);
    }
    println!("Answer 1: {}", sum);

    let mut sum2 = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, c) in b.iter().enumerate() {
            sum2 += (i + 1) * (j + 1) * c.1;
        }
    }
    println!("Answer 2: {}", sum2);
}
