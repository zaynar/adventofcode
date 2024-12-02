use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone)]
pub struct State {
    halted: bool,
    pc: i32,
    mem: HashMap<i32, i32>,
}

pub fn load(input: &str) -> State {
    State {
        halted: false,
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
            "halted={} pc={} mem={}",
            self.halted,
            self.pc,
            self.mem.iter().sorted().map(|(_k, v)| v).join(",")
        );
    }

    pub fn step(self: &mut State) {
        let mem = &mut self.mem;
        let mut pc = self.pc;

        let op = mem[&pc];
        match op {
            1 => {
                let a = mem[&(pc + 1)];
                let b = mem[&(pc + 2)];
                let c = mem[&(pc + 3)];
                mem.insert(c, mem[&a] + mem[&b]);
                pc += 4;
            }
            2 => {
                let a = mem[&(pc + 1)];
                let b = mem[&(pc + 2)];
                let c = mem[&(pc + 3)];
                mem.insert(c, mem[&a] * mem[&b]);
                pc += 4;
            }
            99 => {
                self.halted = true;
            }
            _ => panic!("Unknown opcode {} at {}", op, pc),
        }

        self.pc = pc;
    }

    pub fn execute(self: &mut State) {
        // self.dump();
        while !self.halted {
            self.step();
            // self.dump();
        }
    }
}
