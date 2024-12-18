// Part 1: 6 mins
// Part 1+2: 7 mins

fn run(title: &str, input: &str) {
    let data: Vec<(i32, i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let (id, rest) = line.split_once(" @ ").unwrap();
            let (x, rest) = rest.split_once(",").unwrap();
            let (y, rest) = rest.split_once(": ").unwrap();
            let (w, h) = rest.split_once("x").unwrap();
            (id[1..].parse().unwrap(), x.parse().unwrap(), y.parse().unwrap(), w.parse().unwrap(), h.parse().unwrap())
        })
        .collect();

    let mut grid = Vec::new();
    grid.resize(1000*1000, 0);
    for &(id, x, y, w, h) in &data {
        for dy in 0..h {
            for dx in 0..w {
                grid[(x+dx + (y+dy)*1000) as usize] += 1;
            }
        }
    }

    println!("{} part 1: {}", title, grid.iter().filter(|n| **n > 1).count());

    for &(id, x, y, w, h) in &data {
        let mut ok = true;
        for dy in 0..h {
            for dx in 0..w {
                if grid[(x+dx + (y+dy)*1000) as usize] > 1 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("{} part 2: {}", title, id);
        }
    }
}

const INPUT_DEMO: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
