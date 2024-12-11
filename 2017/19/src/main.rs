// Part 1: 13 mins
// Part 1+2: 14 mins

fn run(title: &str, input: &str) {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut x = data[0].iter().position(|c| *c == '|').unwrap() as i32;
    let mut y: i32 = 0;
    let mut d = (0, 1);

    let mut paths = Vec::new();

    for i in 0.. {
        x += d.0;
        y += d.1;
        // println!("{} {} = {}", x, y, data[y as usize][x as usize]);
        if data[y as usize][x as usize] == ' ' {
            println!("{} part 1: {}", title, paths.iter().collect::<String>());
            println!("{} part 2: {}", title, i + 1);
            break;
        }
        if ('A'..='Z').contains(&data[y as usize][x as usize]) {
            paths.push(data[y as usize][x as usize]);
        }
        if data[y as usize][x as usize] == '+' {
            let da = (-d.1, d.0);
            let db = (d.1, -d.0);
            if y+da.1 < data.len() as i32 && match data[(y + da.1) as usize].get((x + da.0) as usize).unwrap_or(&' ') { 'A'..='Z' | '|' | '-' => true, _ => false } {
                d = da;
            } else {
                d = db;
            }
            // println!("turn {:?}", d);
        }
    }
}

const INPUT_DEMO: &str = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+

";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
