mod intcode;

fn run(title: &str, input: &str) {
    let mut initial = intcode::load(input);
    let mut out = -1;
    initial.execute(|| 1, |n| { println!("{}", n); out = n });

    println!("{} part 1: {}", title, out);
}

fn run2(title: &str, input: &str) {
    let mut initial = intcode::load(input);
    let mut out = -1;
    initial.execute(|| 2, |n| { println!("{}", n); out = n });

    println!("{} part 2: {}", title, out);
}

fn main() {
    run("demo 1", "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    run("demo 2", "1102,34915192,34915192,7,4,7,99,0");
    run("demo 3", "104,1125899906842624,99");
    run("input", &std::fs::read_to_string("09/input.txt").unwrap());
    run2("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
