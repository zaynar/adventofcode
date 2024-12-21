// Part 1: 3 mins
// Part 1+2: 5 mins

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let grid = Grid::from(input);

    let mut part1 = 0;
    let mut a = 0_i64;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    for y in 0..grid.height() {
        if *grid.get((y*3) % grid.width(), y) == '#' {
            part1 += 1;
        }

        if *grid.get((y*1) % grid.width(), y) == '#' {
            a += 1;
        }
        if *grid.get((y*3) % grid.width(), y) == '#' {
            b += 1;
        }
        if *grid.get((y*5) % grid.width(), y) == '#' {
            c += 1;
        }
        if *grid.get((y*7) % grid.width(), y) == '#' {
            d += 1;
        }
        if y*2 < grid.height() && *grid.get(y % grid.width(), y*2) == '#' {
            e += 1;
        }
    }
    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, a*b*c*d*e);


}

const INPUT_DEMO: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("03/input.txt").unwrap());
}
