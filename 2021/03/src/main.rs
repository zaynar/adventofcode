// Part 1: 4 mins
// Part 1+2: 15 mins

use itertools::Itertools;

fn o2(vals: Vec<&str>, i: usize, bits: usize) -> &str {
    println!("o2 {} {:?}", bits, vals);
    if vals.len() == 1 {
        println!("o2 {:?}", vals);
        vals[0]
    } else {
        let ones = vals.iter().filter(|v| v.as_bytes()[i] == b'1').count();
        // println!("{}/{} ones", ones, vals.len());
        if ones * 2 >= vals.len() {
            o2(vals.iter().filter(|v| v.as_bytes()[i] == b'1').cloned().collect_vec(), i + 1, bits)
        } else {
            o2(vals.iter().filter(|v| v.as_bytes()[i] == b'0').cloned().collect_vec(), i + 1, bits)
        }
    }

}

fn co2(vals: Vec<&str>, i: usize, bits: usize) -> &str {
    println!("co2 {} {:?}", bits, vals);
    if vals.len() == 1 {
        println!("co2 {:?}", vals);
        vals[0]
    } else {
        let ones = vals.iter().filter(|v| v.as_bytes()[i] == b'1').count();
        // println!("{}/{} ones", ones, vals.len());
        if ones * 2 >= vals.len() {
            co2(vals.iter().filter(|v| v.as_bytes()[i] == b'0').cloned().collect_vec(), i + 1, bits)
        } else {
            co2(vals.iter().filter(|v| v.as_bytes()[i] == b'1').cloned().collect_vec(), i + 1, bits)
        }
    }

}

fn run(title: &str, input: &str, bits: usize) {
    let mut ones = Vec::new();
    ones.resize(bits, 0);

    let n = input.lines().count();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..bits {
        if ones[i] > n/2 {
            gamma |= 1 << (bits - 1 - i);
        } else {
            epsilon |= 1 << (bits - 1 - i);
        }
    }

    println!("{} part 1: {}", title, gamma * epsilon);

    let o = o2(input.lines().collect_vec(), 0, bits);
    let c = co2(input.lines().collect_vec(), 0, bits);

    println!("{} part 2: {} {} = {}", title, o, c, u32::from_str_radix(o, 2).unwrap() * u32::from_str_radix(c, 2).unwrap());
}

const INPUT_DEMO: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

fn main() {
    run("demo", INPUT_DEMO, 5);
    run("input", &std::fs::read_to_string("03/input.txt").unwrap(), 12);
}
