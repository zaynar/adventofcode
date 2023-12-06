fn main() {
    // let races = vec![(7, 9), (15, 40), (30, 200)];
    // let races = vec![(62, 553), (64, 1010), (91, 1473), (90, 1074)];
    let races = vec![(62649190u64, 553101014731074u64)];

    let mut counts = Vec::new();

    for (time, dist) in races {
        let results = (0..time).map(|hold| (time - hold) * hold);
        // println!("{:?}", results.collect::<Vec<_>>());
        let count = results.filter(|&d| d >= dist).count();
        println!("{}", count);
        counts.push(count);
    }
    println!("{}", counts.iter().product::<usize>());
}
