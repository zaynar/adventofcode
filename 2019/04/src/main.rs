fn main() {
    let mut part1 = 0;
    for i in 284639..=748759 {
        let digits = [
            (i / 100000) % 10,
            (i / 10000) % 10,
            (i / 1000) % 10,
            (i / 100) % 10,
            (i / 10) % 10,
            i % 10,
        ];

        if digits.windows(2).any(|w| w[0] == w[1]) && digits.windows(2).all(|w| w[0] <= w[1]) {
            part1 += 1;
        }
    }

    let mut part2 = 0;
    for i in 284639..=748759 {
        let digits = [
            -1,
            (i / 100000) % 10,
            (i / 10000) % 10,
            (i / 1000) % 10,
            (i / 100) % 10,
            (i / 10) % 10,
            i % 10,
            i32::MAX,
        ];

        if digits.windows(4).any(|w| w[0] != w[1] && w[1] == w[2] && w[2] != w[3]) && digits.windows(2).all(|w| w[0] <= w[1]) {
            part2 += 1;
        }
    }


    println!("part 1: {}", part1);

    println!("part 2: {}", part2);
}
