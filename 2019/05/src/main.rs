mod intcode;

fn run(title: &str, input: &str) {
    let mut initial = intcode::load(input);

    {
        let mut state = initial.clone();
        let mut outputs = Vec::new();
        state.execute(|| 1, |n| outputs.push(n));

        println!("{} part 1: {}", title, outputs.last().unwrap());
    }

    {
        let mut state = initial.clone();
        let mut outputs = Vec::new();
        state.execute(|| 5, |n| outputs.push(n));

        println!("{} part 2: {}", title, outputs.last().unwrap());
    }
}

fn main() {
    if true {
        let mut state = intcode::load("1002,4,3,4,33");
        state.execute_verbose(|| panic!("input"), |_| panic!("output"));
        println!();
    }

    if true {
        let mut state = intcode::load("1101,100,-1,4,0");
        state.execute_verbose(|| panic!("input"), |_| panic!("output"));
        println!();
    }

    if true {
        let mut state = intcode::load("3,2,-1");
        state.execute_verbose(|| 99, |_| panic!("output"));
        println!();
    }

    if true {
        let mut state = intcode::load("4,2,99");
        let mut r = Vec::new();
        state.execute_verbose(|| panic!("input"), |v| r.push(v));
        println!("- {:?}", r);
        println!();
    }

    if true {
        let initial = intcode::load("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");

        for i in 7..=9 {
            let mut state = initial.clone();
            let mut r = Vec::new();
            state.execute(|| i, |v| r.push(v));
            println!("- {} {:?}", i, r);
        }
        println!();
    }

    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
