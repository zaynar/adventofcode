// Part 1: 9 mins
// Part 1+2: 18 mins

use std::collections::HashMap;

fn run(title: &str, input: &str) {

    // 00  10  20
    //   01  11  21
    //     02  12  22

    let mut grid = HashMap::new();

    for line in input.lines() {
        let (mut x, mut y) = (0, 0);
        let mut ch = line.chars();
        while let Some(c) = ch.next() {

            let (dx, dy) = match c {
                'w' => (-1, 0),
                'e' => (1, 0),
                'n' => {
                    match ch.next().unwrap() {
                        'w' => (0, -1),
                        'e' => (1, -1),
                        _ => panic!(),
                    }
                }
                's' => {
                    match ch.next().unwrap() {
                        'w' => (-1, 1),
                        'e' => (0, 1),
                        _ => panic!(),
                    }
                }
                _ => panic!()
            };

            x += dx;
            y += dy;
        }
        *grid.entry((x, y)).or_insert(false) ^= true;
    }

    println!("{} part 1: {}", title, grid.values().filter(|v| **v).count());

    // println!("{:?}", grid.keys());

    let reps = 100;
    let range = reps + 20;
    for i in 0..reps {

        let mut new = HashMap::new();
        for y in -range..=range {
            for x in -range..=range {

                let n = [
                    (-1, 0), (1, 0), (0, -1), (1, -1), (-1, 1), (0, 1)
                ].iter().filter(|(dx, dy)| grid.get(&(x + dx, y + dy)) == Some(&true)).count();

                // false=white, true=black
                if grid.get(&(x, y)) == Some(&true) {
                    if n == 0 || n > 2 {
                        // flip to white
                    } else {
                        new.insert((x, y), true);
                    }
                } else {
                    if n == 2 {
                        new.insert((x, y), true);
                    }
                }
            }
        }

        grid = new;

        println!("{} day {}: {}", title, i+1, grid.values().filter(|v| **v).count());

    }

    println!("{} part 2: {}", title, grid.values().filter(|v| **v).count());
}

const INPUT_DEMO: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("24/input.txt").unwrap());
}
