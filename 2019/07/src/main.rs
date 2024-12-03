use intcode::RunState;
use itertools::Itertools;

mod intcode;

fn run1(initial: &intcode::State, settings: Vec<i32>) -> i32 {
    let mut v = 0;
    for i in 0..=4 {
        let mut amp = initial.clone();
        let mut out = None;
        let mut ins = vec![v, settings[i]];
        amp.execute(|| ins.pop().unwrap(), |n| out = Some(n));
        v = out.unwrap();
    }
    v
}

fn run2(initial: &intcode::State, settings: Vec<i32>) -> i32 {
    let mut v = 0;
    let mut amps = (0..=4).map(|i| (i, initial.clone())).collect_vec();

    for (i, amp) in &mut amps {
        while amp.runstate == RunState::Ready {
            amp.step();
        }
        assert!(amp.runstate == RunState::NeedsInput);
        amp.runstate = RunState::HasInput(settings[*i]);
    }

    let mut outputs = vec![None, None, None, None, Some(0)];
    let mut thrusters = 0;

    while !amps.iter().all(|(_, amp)| amp.runstate == RunState::Halted) {
        // println!("{:?}", amps.iter().map(|(_, amp)| amp.runstate).collect_vec());

        for (i, amp) in &mut amps {
            match amp.runstate {
                RunState::HasOutput(n) => {
                    assert!(outputs[*i].is_none());
                    outputs[*i] = Some(n);
                    if *i == 4 {
                        thrusters = n;
                    }
                    amp.runstate = RunState::Ready;
                }
                RunState::NeedsInput => {
                    let out = &mut outputs[(*i + 1) % 5];
                    if let Some(n) = *out {
                        amp.runstate = RunState::HasInput(n);
                        *out = None;
                    }
                }
                RunState::Ready | RunState::HasInput(_) => amp.step(),
                RunState::Halted => (),
            }
        }
    }

    thrusters
}

fn part1(title: &str, input: &str) {
    let mut initial = intcode::load(input);

    let mut best1 = 0;
    for phases in (0..=4).permutations(5) {
        best1 = best1.max(run1(&initial, phases));
    }

    println!("{} part 1: {}", title, best1);
}

fn part2(title: &str, input: &str) {
    let mut initial = intcode::load(input);

    let mut best2 = 0;
    for phases in (5..=9).permutations(5) {
        best2 = best2.max(run2(&initial, phases));
    }

    println!("{} part 2: {}", title, best2);
}

fn main() {
    part1("demo 1", "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    part1("demo 2", "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    part1("demo 3", "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    part1("input", &std::fs::read_to_string("07/input.txt").unwrap());

    part2("demo 4", "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    part2("demo 5", "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    part2("input", &std::fs::read_to_string("07/input.txt").unwrap());
}
