fn run(title: &str, input: &str, reps: usize) {
    let mut data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let w = data[0].len() as i32;
    let h = data.len() as i32;

    data[0][0] = true;
    data[0][w as usize - 1] = true;
    data[h as usize-1][0] = true;
    data[h as usize-1][w as usize-1] = true;

    for i in 0..reps {

        let mut new = data.clone();
        for y in 0..h {
            for x in 0..w {
                let mut n = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if (dx, dy) != (0, 0) && 0 <= x+dx && x+dx < w && 0 <= y+dy && y+dy < h && data[(y+dy) as usize][(x+dx) as usize] {
                            n += 1;
                        }
                    }
                }
                if data[y as usize][x as usize] {
                    new[y as usize][x as usize] = n == 2 || n == 3;
                } else {
                    new[y as usize][x as usize] = n == 3;
                }
            }
        }

        // println!("{:?}", new);

        data = new;

        data[0][0] = true;
        data[0][w as usize - 1] = true;
        data[h as usize-1][0] = true;
        data[h as usize-1][w as usize-1] = true;
    }

    println!("{} part N: {:?}", title, data.iter().map(|r| r.iter().filter(|c| **c).count()).sum::<usize>());
}

const INPUT_DEMO: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..
";

fn main() {
    run("demo", INPUT_DEMO, 5);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap(), 100);
}
