use std::fs;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    springs_damaged: usize,
    springs_operational: usize,
    springs_unknown: usize,
    counts: Vec<usize>,
    counts_damaged: usize,
    counts_operational: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Damaged, // #
    Operational, // .
    Unknown, // ?
}

// fn is_valid(record: &Record) -> bool {
//     // let mut groups = Vec::new();

//     assert!(record.springs_damaged + record.springs_operational + record.springs_unknown == record.springs.len());
//     if record.springs_damaged != record.counts_damaged {
//         return false;
//     }

//     let mut g = 0;
//     let mut start = None;
//     for (i, &s) in record.springs.iter().enumerate() {
//         if s == Spring::Operational && start.is_some() {
//             if record.counts.get(g).copied() != Some(i - start.unwrap()) {
//                 return false;
//             }
//             g += 1;
//             start = None;
//         } else if s == Spring::Damaged && start.is_none() {
//             start = Some(i);
//         }
//     }
//     if start.is_some() {
//         if record.counts.get(g).copied() != Some(record.springs.len() - start.unwrap()) {
//             return false;
//         }
//         g += 1;
//     }
//     if g != record.counts.len() {
//         return false;
//     }
// // println!("{:?} {:?}", groups, record.counts);
//     return true;
// }

fn is_maybe_valid(record: &Record) -> bool {

    assert!(record.springs_damaged + record.springs_operational + record.springs_unknown == record.springs.len());
    if record.springs_damaged > record.counts_damaged {
        return false;
    }
    if record.springs_operational > record.counts_operational {
        return false;
    }

    let mut g = 0;
    let mut start = None;
    for (i, &s) in record.springs.iter().enumerate() {
        if s == Spring::Unknown {
            return true;
        }
        if s == Spring::Operational && start.is_some() {
            if record.counts.get(g).copied() != Some(i - start.unwrap()) {
                return false;
            }
            g += 1;
            start = None;
        } else if s == Spring::Damaged && start.is_none() {
            start = Some(i);
        }
    }
    if start.is_some() {
        if record.counts.get(g).copied() != Some(record.springs.len() - start.unwrap()) {
            return false;
        }
        g += 1;
    }
    if g != record.counts.len() {
        return false;
    }
    // println!("{:?} {:?}", groups, record.counts);
    return true;
}

fn expand(record: &mut Record) -> usize {
    // println!("-{:?}", record);
    if !is_maybe_valid(record) {
        return 0;
    }

    let unk = record.springs.iter().position(|&s| s == Spring::Unknown);
    let mut count = 0;
    // println!("#{:?}", unk);
    match unk {
        Some(p) => {
            record.springs[p] = Spring::Damaged;
            record.springs_unknown -= 1;
            record.springs_damaged += 1;
            count += expand(record);
            record.springs[p] = Spring::Operational;
            record.springs_damaged -= 1;
            record.springs_operational += 1;
            count += expand(record);
            record.springs[p] = Spring::Unknown;
            record.springs_operational -= 1;
            record.springs_unknown += 1;
        },
        None => {
            assert!(record.springs_unknown == 0);
            // let valid = is_valid(&record);
            // if valid {
                count += 1;
            // }
            // println!(" {:?} {}", record, valid);
        }
    }
    count
}

fn repeat(v: &Vec<Spring>) -> Vec<Spring> {
    [v.clone(), v.clone(), v.clone(), v.clone(), v.clone()].join(&Spring::Unknown)
}

fn main() {
    let mut records: Vec<_> = fs::read_to_string("input").unwrap().lines().map(|line| {
        let mut it = line.split_whitespace();
        let springs: Vec<_> = it.next().unwrap().chars().map(|c| {
            match c {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            }
        }).collect();
        let springs_damaged = springs.iter().filter(|&&s| s == Spring::Damaged).count();
        let springs_operational = springs.iter().filter(|&&s| s == Spring::Operational).count();
        let springs_unknown = springs.iter().filter(|&&s| s == Spring::Unknown).count();
        let counts: Vec<_> = it.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
        let counts_damaged = counts.iter().sum();
        let counts_operational = springs.len() - counts.iter().sum::<usize>();
        Record { springs, springs_damaged, springs_operational, springs_unknown, counts, counts_damaged, counts_operational }
    }).collect();

    // Part 2:
    let mut records: Vec<_> = records.iter().map(|r| {
        Record {
            springs: repeat(&r.springs),
            springs_damaged: r.springs_damaged * 5,
            springs_operational: r.springs_operational * 5,
            springs_unknown: r.springs_unknown * 5 + 4,
            counts: r.counts.repeat(5),
            counts_damaged: r.counts_damaged * 5,
            counts_operational: r.counts_operational * 5 + 4,
        }
    }).collect();

    let sum: usize = records.par_iter_mut().map(|mut record| {
        let count = expand(&mut record);
        println!("{:?} {}", record, count);
        count
        // break;
    }).sum();
    println!("Answer: {}", sum);

    // println!("{:?}", records);
}
