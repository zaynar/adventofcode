use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RunState {
    Ready,
    Halted,
    NeedsInput,
    HasInput(i32),
    HasOutput(i32),
}

#[derive(Clone)]
pub struct State {
    pub runstate: RunState,
    verbose: bool,
    pc: i32,
    mem: HashMap<i32, i32>,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Position(i32),
    Immediate(i32),
}

pub fn load(input: &str) -> State {
    State {
        runstate: RunState::Ready,
        verbose: false,
        pc: 0,
        mem: input
            .trim()
            .split(",")
            .enumerate()
            .map(|(i, n)| (i as i32, str::parse(n).unwrap()))
            .collect(),
    }
}

impl State {
    pub fn set(self: &mut State, i: i32, v: i32) {
        self.mem.insert(i, v);
    }

    pub fn get(self: &mut State, i: i32) -> i32 {
        *self.mem.get(&i).unwrap()
    }

    pub fn dump(self: &mut State) {
        println!(
            "{:?} pc={} mem={}",
            self.runstate,
            self.pc,
            self.mem.iter().sorted().map(|(_k, v)| v).join(",")
        );
    }

    fn operands(self: &mut State, num: usize) -> Vec<Operand> {
        let op = self.mem[&self.pc];
        (0..num)
            .map(|i| {
                let mode = (op / 10_i32.pow(2 + i as u32)) % 10;
                match mode {
                    0 => Operand::Position(self.mem[&(self.pc + 1 + i as i32)]),
                    1 => Operand::Immediate(self.mem[&(self.pc + 1 + i as i32)]),
                    _ => panic!("Invalid op mode"),
                }
            })
            .collect()
    }

    fn oset(self: &mut State, o: Operand, val: i32) {
        if self.verbose {
            println!("oset {:?} {}", o, val);
        }
        match o {
            Operand::Position(n) => self.mem.insert(n, val),
            Operand::Immediate(n) => panic!("Setting immediate"),
        };
    }

    fn oget(self: &mut State, o: Operand) -> i32 {
        match o {
            Operand::Position(n) => *self.mem.get(&n).unwrap(),
            Operand::Immediate(n) => n,
        }
    }

    pub fn step(self: &mut State)
    {
        let mem = &mut self.mem;
        let mut pc = self.pc;

        let op = mem[&pc];
        if self.verbose {
            println!("step pc={} op={}", pc, op);
        }
        match op % 100 {
            1 => { // add
                let os = self.operands(3);
                if self.verbose {
                    println!("add [{:?}]", os);
                }
                let r = self.oget(os[0]) + self.oget(os[1]);
                self.oset(os[2], r);
                pc += 4;
            }

            2 => { // mul
                let os = self.operands(3);
                if self.verbose {
                    println!("mul [{:?}]", os);
                }
                let r = self.oget(os[0]) * self.oget(os[1]);
                self.oset(os[2], r);
                pc += 4;
            }

            3 => { // input
                match self.runstate {
                    RunState::HasInput(input) => {
                        let os = self.operands(1);
                        if self.verbose {
                            println!("input [{:?}] = {}", os, input);
                        }
                        self.oset(os[0], input);
                        pc += 2;
                        self.runstate = RunState::Ready;
                    }
                    RunState::Ready => {
                        self.runstate = RunState::NeedsInput;
                    }
                    _ => {
                        panic!("input in invalid runstate {:?}", self.runstate);
                    }
                }
            }

            4 => { // output
                match self.runstate {
                    RunState::Ready => {
                        let os = self.operands(1);
                        let v = self.oget(os[0]);
                        if self.verbose {
                            println!("output [{:?}] = {}", os, v);
                        }
                        self.runstate = RunState::HasOutput(v);
                        pc += 2;
                    }
                    _ => {
                        panic!("output in invalid runstate {:?}", self.runstate);
                    }
                }

            }

            5 => { // jump-if-true
                let os = self.operands(2);
                let v = self.oget(os[0]);
                if v != 0 {
                    pc = self.oget(os[1]);
                } else {
                    pc += 3;
                }
            }

            6 => { // jump-if-false
                let os = self.operands(2);
                let v = self.oget(os[0]);
                if v == 0 {
                    pc = self.oget(os[1]);
                } else {
                    pc += 3;
                }
            }

            7 => { // less-than
                let os = self.operands(3);
                let v = self.oget(os[0]) < self.oget(os[1]);
                self.oset(os[2], if v { 1 } else { 0 });
                pc += 4;
            }

            8 => { // equals
                let os = self.operands(3);
                let v = self.oget(os[0]) == self.oget(os[1]);
                self.oset(os[2], if v { 1 } else { 0 });
                pc += 4;
            }

            99 => {
                self.runstate = RunState::Halted;
            }
            _ => panic!("Unknown opcode {} at {}", op, pc),
        }

        self.pc = pc;
    }

    pub fn execute<FIn, FOut>(self: &mut State, mut input: FIn, mut output: FOut)
    where
        FIn: FnMut() -> i32,
        FOut: FnMut(i32) -> (),
    {
        if self.verbose {
            self.dump();
        }
        loop {
            match self.runstate {
                RunState::Halted => return,
                RunState::Ready => self.step(),
                RunState::HasInput(_) => self.step(),
                RunState::NeedsInput => {
                    self.runstate = RunState::HasInput(input());
                }
                RunState::HasOutput(n) => {
                    output(n);
                    self.runstate = RunState::Ready;
                }
            }
            if self.verbose {
                self.dump();
            }
        }
    }

    pub fn execute_verbose<FIn, FOut>(self: &mut State, input: FIn, output: FOut)
    where
        FIn: FnMut() -> i32,
        FOut: FnMut(i32) -> (),
    {
        self.verbose = true;
        self.execute(input, output);
        self.verbose = false;
    }
}
