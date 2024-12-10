// Part 1: 2 mins
// Part 1+2: 3 mins

fn run(title: &str, input: &str) {
    let mut data: Vec<i32> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut pc: i32 = 0;
    let mut steps = 0;
    while pc >= 0 && pc < data.len() as i32 {
        let n = data[pc as usize];
        data[pc as usize] += 1;
        pc += n;
        steps += 1;
    }

    println!("{} part 1: {}", title, steps);

    let mut data: Vec<i32> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut pc: i32 = 0;
    let mut steps = 0;
    while pc >= 0 && pc < data.len() as i32 {
        let n = data[pc as usize];
        if n >= 3 {
            data[pc as usize] -= 1;
        } else {
            data[pc as usize] += 1;
        }
        pc += n;
        steps += 1;
    }

    println!("{} part 2: {}", title, steps);
}

const INPUT_DEMO: &str = "0
3
0
1
-3
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
