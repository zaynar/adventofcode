fn demo(title: &str) {
    let is = [
        [-1, -2, 6, 3, 8],
        [2, 3, -2, -1, 3],
    ];

    let mut max_score = 0;
    for a in 0..=100 {
        let b = 100 - a;
        // println!("{} {}", a, b);
        assert_eq!(a+b, 100);
        let score = (0..4).map(|i|
            (is[0][i] * a + is[1][i] * b).max(0)
        ).product::<i64>();
        max_score = max_score.max(score);
    }

    println!("{} part 1: {}", title, max_score);
    println!("{} part 2: {}", title, "TODO");
}

fn run(title: &str) {
    let is = [
        [5,  -1,  0,  0,  5],
        [-1,  3,  0,  0,  1],
        [0,  -1,  4,  0,  6],
        [-1,  0,  0,  2,  8],
    ];

    let mut max_score = 0;
    let mut max_score2 = 0;
    for a in 0..=100 {
        for b in 0..=(100-a) {
            for c in 0..(100-a-b) {
                let d = 100-a-b-c;
                assert_eq!(a+b+c+d, 100);
                let score = (0..4).map(|i|
                    (is[0][i] * a + is[1][i] * b + is[2][i] * c + is[3][i] * d).max(0)
                ).product::<i64>();
                max_score = max_score.max(score);

                let i = 4;
                if (is[0][i] * a + is[1][i] * b + is[2][i] * c + is[3][i] * d) == 500 {
                    max_score2 = max_score2.max(score);
                }
            }
        }
    }

    println!("{} part 1: {}", title, max_score);
    println!("{} part 2: {}", title, max_score2);
}

fn main() {
    demo("demo");
    run("input");
}
