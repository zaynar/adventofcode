// Part 1: 7 mins
// Part 1+2: 10 mins

fn sev(data: &Vec<(i32, i32)>, t: i32) -> i32 {
    let mut s = 0;
    for (d, r) in data {
        if (t + d) % (r * 2 - 2) == 0 {
            s += d * r;
        }
    }
    s
}

fn bad(data: &Vec<(i32, i32)>, t: i32) -> bool {
    let mut s = 0;
    for (d, r) in data {
        if (t + d) % (r * 2 - 2) == 0 {
            return true;
        }
    }
    false
}
fn run(title: &str, input: &str) {
    let data: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            (a, b)
        })
        .collect();

    println!("{} part 1: {}", title, sev(&data, 0));

    for j in 0.. {
        // println!(" {} {}", j, bad(&data, j));
        if !bad(&data, j)  {
            println!("{} part 2: {}", title, j);
            break;
        }
    }

}

const INPUT_DEMO: &str = "0: 3
1: 2
4: 4
6: 4
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("13/input.txt").unwrap());
}
