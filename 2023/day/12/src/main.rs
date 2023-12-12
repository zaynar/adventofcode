use std::{fs, collections::HashMap};
use rayon::prelude::*;
use std::sync::atomic::AtomicUsize;

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

#[derive(Clone, PartialEq, Eq, Hash)]
struct ValidState {
    i: usize,
    g: usize,
    start: Option<usize>,
}

fn is_maybe_valid(record: &Record, state: &mut ValidState) -> bool {

    assert!(record.springs_damaged + record.springs_operational + record.springs_unknown == record.springs.len());
    if record.springs_damaged > record.counts_damaged {
        return false;
    }
    if record.springs_operational > record.counts_operational {
        return false;
    }

    // let mut g = 0;
    // let mut start = None;
    for (i, &s) in record.springs.iter().enumerate().skip(state.i) {
        state.i = i;
        if s == Spring::Unknown {
            return true;
        }
        if s == Spring::Operational && state.start.is_some() {
            if record.counts.get(state.g).copied() != Some(i - state.start.unwrap()) {
                return false;
            }
            state.g += 1;
            state.start = None;
        } else if s == Spring::Damaged && state.start.is_none() {
            state.start = Some(i);
        }
    }
    if state.start.is_some() {
        if record.counts.get(state.g).copied() != Some(record.springs.len() - state.start.unwrap()) {
            return false;
        }
        state.g += 1;
    }
    if state.g != record.counts.len() {
        return false;
    }
    // println!("{:?} {:?}", groups, record.counts);
    return true;
}

// Memoise based on (next_unkown, state)

fn expand(record: &mut Record, state: &mut ValidState, memo: &mut HashMap<(usize, ValidState), usize>) -> usize {
    // println!("-{:?}", record);
    let mut state = state.clone();
    if !is_maybe_valid(record, &mut state) {
        return 0;
    }

    if record.springs_unknown == 0 {
        return 1;
    }

    let unk = record.springs.iter().position(|&s| s == Spring::Unknown).unwrap();

    let memo_key = (unk, state.clone());
    let memo_entry = memo.get(&memo_key);
    if let Some(&count) = memo_entry {
        return count;
    }

    let mut count = 0;
    // println!("#{:?}", unk);

    let p = unk;
    record.springs[p] = Spring::Damaged;
    record.springs_unknown -= 1;
    record.springs_damaged += 1;
    count += expand(record, &mut state, memo);
    record.springs[p] = Spring::Operational;
    record.springs_damaged -= 1;
    record.springs_operational += 1;
    count += expand(record, &mut state, memo);
    record.springs[p] = Spring::Unknown;
    record.springs_operational -= 1;
    record.springs_unknown += 1;

    memo.insert(memo_key, count);

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

    let status = AtomicUsize::new(0);
    let sum: usize = records.par_iter_mut().map(|mut record| {
        let mut memo = HashMap::new();
        let mut state = ValidState { i: 0, g: 0, start: None };
        let count = expand(&mut record, &mut state, &mut memo);
        println!("<<<{}>>> {:?} {}", status.fetch_add(1, std::sync::atomic::Ordering::SeqCst), record, count);
        count
        // break;
    }).sum();
    println!("Answer: {}", sum);

    // println!("{:?}", records);
}
