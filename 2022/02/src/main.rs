// Part 1: 5 mins
// Part 1+2: 6 mins

fn run(title: &str, input: &str) {
    let mut part1 = 0;

    for line in input.lines() {
        let (abc, xyz) = line.split_once(" ").unwrap();
        let outcome = match (abc, xyz) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            _ => panic!(),
        };
        let score = match xyz {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!(),
        };
        part1 += outcome + score;
    }

    println!("{} part 1: {}", title, part1);

    let mut part2 = 0;

    for line in input.lines() {
        let (abc, xyz) = line.split_once(" ").unwrap();
        let outcome = match (abc, xyz) {
            ("A", "X") => 3,
            ("A", "Y") => 1,
            ("A", "Z") => 2,
            ("B", "X") => 1,
            ("B", "Y") => 2,
            ("B", "Z") => 3,
            ("C", "X") => 2,
            ("C", "Y") => 3,
            ("C", "Z") => 1,
            _ => panic!(),
        };
        let score = match xyz {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!(),
        };
        part2 += outcome + score;
    }

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
