// Part 1: 12 mins
// Part 1+2: 15 mins

extern crate nalgebra as na;
use std::collections::HashMap;

use itertools::Itertools;
use na::Vector3;

#[derive(Debug, Clone)]
struct Particle {
    p: Vector3<i64>,
    v: Vector3<i64>,
    a: Vector3<i64>,
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i64
            = n:$("-"? ['0'..='9']*) {? n.parse().or(Err("number")) }

        rule vec() -> Vector3<i64>
            = "<" x:number() "," y:number() "," z:number() ">" { Vector3::new(x, y, z) }

        rule particle() -> Particle
            = "p=" p:vec() ", v=" v:vec() ", a=" a:vec() { Particle {p, v, a} }

        pub rule file() -> Vec<Particle>
            = (p:particle() "\n" { p })*
    }
}

fn run(title: &str, input: &str) {
    let mut data = input_parser::file(input).unwrap();

    println!("{:?}", data);

    for i in 0..10000 {
        let mut pos = HashMap::new();
        for p in data.iter_mut() {
            p.v += p.a;
            p.p += p.v;

            *pos.entry(p.p).or_insert(0) += 1;
        }

        data = data.iter().filter(|p| pos[&p.p] == 1).cloned().collect();
    }

    // println!("{} part 1: {}", title, data.iter().map(|p| p.p.x.abs() + p.p.y.abs() + p.p.z.abs()).position_min().unwrap());

    println!("{} part 2: {}", title, data.len());
}

const INPUT_DEMO: &str = "";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("20/input.txt").unwrap());
}
