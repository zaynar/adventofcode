// Part 1: 13 mins
// Part 1+2: 17 mins

use std::collections::HashMap;

use itertools::Itertools;

fn run(title: &str, input: &str) {

    let mut data = Vec::new();
    for line in input.lines() {
        // 01234567890123456789
        // [1518-11-01 00:00] x
        let ts = (
            line[1..5].parse::<i32>().unwrap(),
            line[6..8].parse::<i32>().unwrap(),
            line[9..11].parse::<i32>().unwrap(),
            line[12..14].parse::<i32>().unwrap(),
            line[15..17].parse::<i32>().unwrap(),
        );
        let cmd = line[19..].to_owned();
        data.push((ts, cmd));
    }
    data.sort();

    let mut sleeps: HashMap<i32, i32> = HashMap::new();
    let mut sleeps_min: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut guard = 0;
    let mut t = 0;
    for (ts, cmd) in &data {
        let ts = ts.3 * 60 + ts.4;
        if cmd.starts_with("Guard") {
            guard = cmd[7..].split_once(' ').unwrap().0.parse::<i32>().unwrap();
        } else if cmd.starts_with("falls") {
            t = ts;
        } else if cmd.starts_with("wakes") {
            *sleeps.entry(guard).or_insert(0) += (ts - t).rem_euclid(3600);

            let min = sleeps_min.entry(guard).or_insert_with(|| vec![0; 60]);

            for dt in 0 .. (ts - t).rem_euclid(3600) {
                min[((t + dt) % 60) as usize] += 1;
            }
        }
    }

    let most = sleeps.iter().sorted_by_key(|(k, v)| **v).last().unwrap();
    println!("{:?}", most.0);
    println!("{:?}", sleeps_min[most.0].iter().position_max().unwrap());

    println!("{} part 1: {}", title, *most.0 as usize * sleeps_min[most.0].iter().position_max().unwrap());

    let part2 = sleeps_min.iter().max_by_key(|(k, v)| v.iter().max().unwrap()).unwrap();
    println!("{} part 2: {:?}", title, *part2.0 as usize * part2.1.iter().position_max().unwrap());
}

const INPUT_DEMO: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
