extern crate nalgebra as na;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use itertools::Itertools;
use na::Vector3;

peg::parser! {
    grammar input_parser() for str {
        rule number() -> i32
            = n:$("-"? ['0'..='9']*<1,3>) {? n.parse().or(Err("number")) }

        rule planet() -> Vector3<i32>
            = "<x=" x:number() ", y=" y:number() ", z=" z:number() ">" { Vector3::new(x, y, z) }

        pub rule planets() -> Vec<Vector3<i32>>
            = (p:planet() "\n" { p })*
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Planet {
    pos: Vector3<i32>,
    vel: Vector3<i32>,
}

pub fn lcm(nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

pub fn gcd(nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = gcd(&nums[1..]);
    gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn run(title: &str, input: &str) {
    let data = input_parser::planets(input).unwrap();

    let mut planets = data
        .iter()
        .map(|&pos| Planet {
            pos,
            vel: Vector3::zeros(),
        })
        .collect_vec();

    for i in 0..1000 {
        // println!("{} {:?}", i, planets);

        for n in 0..planets.len() {
            for m in 0..planets.len() {
                if n == m {
                    continue;
                }
                let b = planets[m].clone();
                let a = &mut planets[n];

                a.vel.x += b.pos.x.cmp(&a.pos.x) as i32;
                a.vel.y += b.pos.y.cmp(&a.pos.y) as i32;
                a.vel.z += b.pos.z.cmp(&a.pos.z) as i32;
            }
        }

        for a in &mut planets {
            a.pos += a.vel;
        }
    }

    let energy: i32 = planets
        .iter()
        .map(|p| p.pos.abs().sum() * p.vel.abs().sum())
        .sum();

    println!("{} part 1: {}", title, energy);
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn run2(title: &str, input: &str) {
    let data = input_parser::planets(input).unwrap();

    let mut planets = data
        .iter()
        .map(|&pos| Planet {
            pos,
            vel: Vector3::zeros(),
        })
        .collect_vec();

    let mut seen_x = HashMap::new();
    let mut seen_y = HashMap::new();
    let mut seen_z = HashMap::new();

    let mut res_x = None;
    let mut res_y = None;
    let mut res_z = None;

    for i in 0..10000000000i64 {
        if i % 10000000 == 0 {
            println!("{} {:?}", i, planets);
        }

        if res_x.is_none() {
            if let Some(n) =
                seen_x.insert(planets.iter().map(|p| (p.pos.x, p.vel.x)).collect_vec(), i)
            {
                println!("X: {} - {} = {}", i, n, i - n);
                res_x = Some(i - n);
            }
        }
        if res_y.is_none() {
            if let Some(n) =
                seen_y.insert(planets.iter().map(|p| (p.pos.y, p.vel.y)).collect_vec(), i)
            {
                println!("Y: {} - {} = {}", i, n, i - n);
                res_y = Some(i - n);
            }
        }
        if res_z.is_none() {
            if let Some(n) =
                seen_z.insert(planets.iter().map(|p| (p.pos.z, p.vel.z)).collect_vec(), i)
            {
                println!("Z: {} - {} = {}", i, n, i - n);
                res_z = Some(i - n);
            }
        }

        if let (Some(x), Some(y), Some(z)) = (res_x, res_y, res_z) {
            println!("### {}", lcm(&[x as i128, y as i128, z as i128]));
            return;
        }

        for n in 0..planets.len() {
            for m in 0..planets.len() {
                if n == m {
                    continue;
                }
                let b = planets[m].clone();
                let a = &mut planets[n];

                a.vel.x += b.pos.x.cmp(&a.pos.x) as i32;
                a.vel.y += b.pos.y.cmp(&a.pos.y) as i32;
                a.vel.z += b.pos.z.cmp(&a.pos.z) as i32;
            }
        }

        for a in &mut planets {
            a.pos += a.vel;
        }
    }

    let energy: i32 = planets
        .iter()
        .map(|p| p.pos.abs().sum() * p.vel.abs().sum())
        .sum();

    println!("{} part 2: {:?}", title, planets);
}

const INPUT_DEMO: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("12/input.txt").unwrap());

    //     run2("demo", "<x=-8, y=-10, z=0>
    // <x=5, y=5, z=10>
    // <x=2, y=-7, z=3>
    // <x=9, y=-8, z=-3>
    // ");
    // run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
