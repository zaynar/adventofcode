// Part 1: 4 mins
// Part 1+2: 9 mins

fn parse(data: &Vec<i32>, mut i: usize, part1: &mut i32) -> usize {
    let numc = data[i];
    let numm = data[i+1];
    i += 2;
    for j in 0..numc {
        i = parse(data, i, part1);
    }
    for j in 0..numm {
        *part1 += data[i];
        i += 1;
    }
    i
}

fn parse2(data: &Vec<i32>, mut i: usize) -> (usize, i32) {
    let numc = data[i];
    let numm = data[i+1];
    i += 2;
    let mut co = Vec::new();
    for j in 0..numc {
        co.push(i);
        i = parse2(data, i).0;
    }
    let mut r = 0;
    for j in 0..numm {
        if numc == 0 {
            r += data[i];
        } else {
            if let Some(c) = co.get(data[i] as usize - 1) {
                r += parse2(data, *c).1;
            }
        }
        i += 1;
    }
    (i, r)
}
fn run(title: &str, input: &str) {
    let data: Vec<i32> = input.trim().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();

    let mut part1 = 0;
    parse(&data, 0, &mut part1);

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {:?}", title, parse2(&data, 0));
}

const INPUT_DEMO: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
