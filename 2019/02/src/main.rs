use intcode;

fn run(title: &str, input: &str) {
    let mut initial = intcode::load(input);

    let mut state = initial.clone();
    state.set(1, 12);
    state.set(2, 2);
    state.execute();
    let p1 = state.get(0);

    println!("{} part 1: {}", title, p1);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut state = initial.clone();
            state.set(1, noun);
            state.set(2, verb);
            state.execute();
            if state.get(0) == 19690720 {
                println!("{} part 2: {}", title, 100 * noun + verb);
            }
        }
    }
}

const INPUT_DEMO: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("02/input.txt").unwrap());
}
