use std::{fs, collections::HashMap};

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Damaged, // #
    Operational, // .
    Unknown, // ?
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ValidState {
    i: usize,
    g: usize,
    start: Option<usize>,
}

fn is_maybe_valid(record: &Record, state: &mut ValidState) -> bool {
    for (i, &s) in record.springs.iter().enumerate().skip(state.i) {
        state.i = i;
        if s == Spring::Unknown {
            // Valid up to this point
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

    return true;
}

fn expand(record: &mut Record, mut state: ValidState, memo: &mut HashMap<(usize, ValidState), usize>) -> usize {
    if !is_maybe_valid(record, &mut state) {
        return 0;
    }

    let unk = record.springs.iter().position(|&s| s == Spring::Unknown);
    if unk.is_none() {
        return 1;
    }
    let unk = unk.unwrap();

    let memo_key = (unk, state);
    let memo_entry = memo.get(&memo_key);
    if let Some(&count) = memo_entry {
        return count;
    }

    let mut count = 0;

    record.springs[unk] = Spring::Damaged;
    count += expand(record, state, memo);
    record.springs[unk] = Spring::Operational;
    count += expand(record, state, memo);
    record.springs[unk] = Spring::Unknown;

    memo.insert(memo_key, count);

    count
}

fn repeat(v: &Vec<Spring>) -> Vec<Spring> {
    [v.clone(), v.clone(), v.clone(), v.clone(), v.clone()].join(&Spring::Unknown)
}

fn main() {
    let mut records: Vec<_> = fs::read_to_string("input").unwrap().lines().map(|line| {
        let (springs, counts) = line.split_once(" ").unwrap();
        let springs: Vec<_> = springs.chars().map(|c| {
            match c {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            }
        }).collect();
        let counts: Vec<_> = counts.split(",").map(|s| s.parse().unwrap()).collect();
        Record { springs, counts }
    }).collect();

    // Part 2:
    let mut records: Vec<_> = records.iter().map(|r| {
        Record {
            springs: repeat(&r.springs),
            counts: r.counts.repeat(5),
        }
    }).collect();

    let sum: usize = records.iter_mut().map(|mut record| {
        let mut memo = HashMap::new();
        let state = ValidState { i: 0, g: 0, start: None };
        expand(&mut record, state, &mut memo)
    }).sum();
    println!("Answer: {}", sum);
}
