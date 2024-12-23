// Part 1: 2 mins
// Part 1+2: 3 mins

fn run(title: &str, input: &str) {
    let mut x = 0;
    let mut d = 0;

    for line in input.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let n: u32 = n.parse().unwrap();
        match dir {
            "forward" => x += n,
            "down" => d += n,
            "up" => d -= n,
            _ => panic!()
        }
    }

    println!("{} part 1: {}", title, x*d);

    let mut aim = 0;
    let mut x = 0;
    let mut d = 0;

    for line in input.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let n: u32 = n.parse().unwrap();
        match dir {
            "forward" => {
                x += n;
                d += aim * n;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!()
        }
    }

    println!("{} part 2: {}", title, x*d);
}

const INPUT_DEMO: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
