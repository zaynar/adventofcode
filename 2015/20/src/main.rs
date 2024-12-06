use rayon::prelude::*;

fn presents(i: u32) -> u32 {
    let mut p = 0;
    for elf in 1..=i {
        if i % elf == 0 {
            p += elf * 10;
        }
    }
    p
}

fn main() {
    for h in 1..10 {
        println!("{} {}", h, presents(h));
    }
    for h in [600_000, 1_000_000, 1_500_000] {
       println!("{} {}", h, presents(h));
    }

    let target = 29000000;

    // let hs: Vec<_> = (0..700_000).into_par_iter().rev().filter(|h| {
    //     if presents(*h) >= target {
    //         println!("part 1: {}", h);
    //         true
    //         // break;
    //     } else {
    //         false
    //     }
    // }).collect();
    // println!("{:?}", hs);

    let mut houses = vec![0; 1_000_000];
    for elf in 1..=houses.len() {
        for i in 1.. {
            if elf * i >= houses.len() {
                break;
            }
            // println!("{} {}", elf, elf*i);
            houses[elf * i] += 10 * elf;
        }
    }
    println!("{:?}", &houses[0..10]);
    println!("part 1: {:?}", houses.iter().position(|p| *p >= target));

    let mut houses = vec![0; 1_000_000];
    for elf in 1..=houses.len() {
        for i in 1..=50 {
            if elf * i >= houses.len() {
                break;
            }
            houses[elf * i] += 11 * elf;
        }
    }

    println!("part 2: {:?}", houses.iter().position(|p| *p >= target));
}
