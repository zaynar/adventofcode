use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RunState {
    Ready,
    Halted,
    NeedsInput,
    HasInput(i64),
    HasOutput(i64),
}

#[derive(Clone)]
pub struct State {
    pub runstate: RunState,
    pub verbose: bool,
    pc: i64,
    relbase: i64,
    mem: HashMap<i64, i64>,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

pub fn load(input: &str) -> State {
    State {
        runstate: RunState::Ready,
        verbose: false,
        pc: 0,
        relbase: 0,
        mem: input
            .trim()
            .split(",")
            .enumerate()
            .map(|(i, n)| (i as i64, str::parse(n).unwrap()))
            .collect(),
    }
}

impl State {
    pub fn set(self: &mut State, i: i64, v: i64) {
        self.mem.insert(i, v);
    }

    pub fn get(self: &mut State, i: i64) -> i64 {
        *self.mem.get(&i).unwrap()
    }

    pub fn dump(self: &mut State) {
        println!(
            "{:?} pc={} rel={} mem={}",
            self.runstate,
            self.pc,
            self.relbase,
            self.mem.iter().sorted().map(|(_k, v)| v).join(",")
        );
    }

    fn operands(self: &mut State, num: usize) -> Vec<Operand> {
        let op = self.mem[&self.pc];
        (0..num)
            .map(|i| {
                let mode = (op / 10_i64.pow(2 + i as u32)) % 10;
                match mode {
                    0 => Operand::Position(self.mem[&(self.pc + 1 + i as i64)]),
                    1 => Operand::Immediate(self.mem[&(self.pc + 1 + i as i64)]),
                    2 => Operand::Relative(self.mem[&(self.pc + 1 + i as i64)]),
                    _ => panic!("Invalid op mode"),
                }
            })
            .collect()
    }

    fn oset(self: &mut State, o: Operand, val: i64) {
        if self.verbose {
            println!("oset {:?} {}", o, val);
        }
        match o {
            Operand::Position(n) => self.mem.insert(n, val),
            Operand::Immediate(_) => panic!("Setting immediate"),
            Operand::Relative(n) => self.mem.insert(self.relbase + n, val),
        };
    }

    fn oget(self: &mut State, o: Operand) -> i64 {
        match o {
            Operand::Position(n) => *self.mem.get(&n).unwrap_or(&0),
            Operand::Immediate(n) => n,
            Operand::Relative(n) => *self.mem.get(&(self.relbase + n)).unwrap_or(&0),
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

            9 => { // set relative base
                let os = self.operands(1);
                self.relbase += self.oget(os[0]);
                pc += 2;
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
        FIn: FnMut() -> i64,
        FOut: FnMut(i64) -> (),
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
        FIn: FnMut() -> i64,
        FOut: FnMut(i64) -> (),
    {
        self.verbose = true;
        self.execute(input, output);
        self.verbose = false;
    }
}
