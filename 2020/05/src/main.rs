// Part 1: 4 mins
// Part 1+2: 5 mins

fn run(title: &str, input: &str) {
    let mut part1 = 0;
    let mut s = [false; 128*8];
    for line in input.lines() {
        let mut r0 = 0;
        let mut r1 = 128;
        let mut c0 = 0;
        let mut c1 = 8;

        for c in line.chars() {
            match c {
                'F' => { (r0, r1) = (r0, (r0+r1)/2) }
                'B' => { (r0, r1) = ((r0+r1)/2, r1) }
                'L' => { (c0, c1) = (c0, (c0+c1)/2) }
                'R' => { (c0, c1) = ((c0+c1)/2, c1) }
                _ => panic!()
            }
        }

        let id = r0*8 + c0;
        // println!("{} {}", line, id);
        part1 = part1.max(id);
        s[id] = true;
    }

    println!("{} part 1: {}", title, part1);

    for i in 0..s.len()-3 {
        if s[i..i+3] == [true, false, true] {
            println!("{} part 2: {}", title, i+1);
        }
    }
}


const INPUT_DEMO: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
