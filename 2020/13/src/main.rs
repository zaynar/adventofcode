// Part 1: 5 mins
// Part 1+2: 41 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (t0, buses) = input.split_once("\n").unwrap();
    let t0 = t0.parse::<i64>().unwrap();
    let buses: Vec<i64> = buses.trim().split(",").filter_map(|n| n.parse().ok()).collect();

    for t in t0.. {
        for b in buses.iter() {
            if t % b == 0 {
                println!("{} part 1: {}", title, (t - t0) * b);
                return;
            }
        }
    }

}

pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

pub fn gcd(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = gcd(&nums[1..]);
    gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn inverse(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let q = r / newr;
        (t, newt) = (newt, t - q * newt);
        (r, newr) = (newr, r - q * newr);
    }

    assert!(r <= 1);
    if t < 0 {
        t += n;
    }

    t
}


fn run2(title: &str, input: &str) {
    let (t0, buses) = input.split_once("\n").unwrap();
    let t0 = t0.parse::<i64>().unwrap();
    let buses: Vec<(usize, i64)> = buses.trim().split(",").enumerate().filter_map(
        |(i, n)| if let Ok(n) = n.parse::<i64>() { Some((i, n)) } else { None }
    ).sorted_by_key(|(i, b)| *b).collect();

    println!("{:?}", buses);

    for (i, b) in buses.iter() {
        println!("t % {} == {}", b, (b - *i as i64).rem_euclid(*b));
    }

    println!("lcm {} - {}", lcm(&buses.iter().map(|(i, b)| *b).collect_vec()), buses.iter().map(|(i, b)| *b).product::<i64>());

    if false {
        let prod = lcm(&buses.iter().map(|(i, b)| *b).collect_vec());

        let mut r = 0;

        for (i, b) in buses.iter() {
            let num = *b;
            let rem = (-(*i as i64)).rem_euclid(*b);
            let pp = prod / num;
            r += rem * inverse(pp, num) * pp;
        }

        println!("{} part 2: {}", title, r % prod);
    } else {

        // Given
        // (1) t % m = o
        // (2) t % b = c   [where c = -i % b]
        // ==>
        // t = o + m*j  satisfies (1)
        // Iterate over j until we find one which satisfies (2)
        // Now we have
        //   t % (m*b) = o + m*j
        // Repeat for each pair

        let mut m: i64 = 1;
        let mut o: i64 = 0;
        for (i, b) in buses.iter() {
            println!();
            println!("t % {} == {}", m, (-o).rem_euclid(m));
            println!("& t % {} == {}", b, (-(*i as i64)).rem_euclid(*b));
            for j in 0..*b {
                if (o + j*m) % b == (-(*i as i64)).rem_euclid(*b) {
                    // println!("# {} % {} = {}", o+j*m, b, (o+j*m)%m);
                    o = o + j*m;
                    break;
                }
            }
            m *= *b;
        }

        println!(">> t % {} == {}", m, (-o).rem_euclid(m));
        println!("{} part 2: {}", title, o);
    }
}



const INPUT_DEMO: &str = "939
7,13,x,x,59,x,31,19
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("13/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("13/input.txt").unwrap());
}
