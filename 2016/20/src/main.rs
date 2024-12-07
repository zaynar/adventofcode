use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut data: Vec<(u64, u64)> = input.lines().map(|line| {
        let (min, max) = line.split_once("-").unwrap();
        (min.parse().unwrap(), max.parse().unwrap())
    }).collect();

    // for i in 0.. {
    //     if !data.iter().any(|&(min, max)| min <= i && i <= max) {
    //         println!("{} part 1: {}", title, i);
    //         break;
    //     }
    // }

    data.sort();

    println!("{:x?}", data);

    let mut i = 0;
    while i + 1 < data.len() {
        // println!("{:?}", data.len());
        let cur = data[i];
        let next = data[i+1];
        if data[i].1 >= data[i+1].0 - 1 {
            data[i].1 = u64::max(data[i].1, data[i+1].1);
            data.remove(i+1);
        } else {
            i += 1;
        }
    }

    println!("{:x?}", data);

    data.push((4294967295+1, 4294967295+1));

    println!("{} part 2: {}", title, data.iter().tuple_windows().map(|(a, b)| b.0 - a.1 - 1).sum::<u64>());

}

fn main() {
    run("input", &std::fs::read_to_string("20/input.txt").unwrap());
}
