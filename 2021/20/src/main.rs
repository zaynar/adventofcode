// Part 1: 14 mins
// Part 1+2: 15 mins

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let (key, grid) = input.split_once("\n\n").unwrap();
    let key = key.chars().map(|c| match c {
        '.' => 0,
        '#' => 1,
        _ => panic!()
    }).collect_vec();

    let mut grid = Grid::from(grid).map(|c| match c {
        '.' => 0,
        '#' => 1,
        _ => panic!()
    });

    // println!("{:?}", key);
    // println!("{:?}", grid);

    for i in 0..50 {
        let default = if i % 2 == 0 { 0 } else { key[0] };
        let mut newgrid = Grid::new_empty(grid.width() as usize + 2, grid.height() as usize + 2, 0);

        newgrid.for_each_mut(|x, y, c| {
            let i =
                (grid.try_get(x - 1 - 1, y - 1 - 1).unwrap_or(&default) << 8) |
                (grid.try_get(x + 0 - 1, y - 1 - 1).unwrap_or(&default) << 7) |
                (grid.try_get(x + 1 - 1, y - 1 - 1).unwrap_or(&default) << 6) |
                (grid.try_get(x - 1 - 1, y + 0 - 1).unwrap_or(&default) << 5) |
                (grid.try_get(x + 0 - 1, y + 0 - 1).unwrap_or(&default) << 4) |
                (grid.try_get(x + 1 - 1, y + 0 - 1).unwrap_or(&default) << 3) |
                (grid.try_get(x - 1 - 1, y + 1 - 1).unwrap_or(&default) << 2) |
                (grid.try_get(x + 0 - 1, y + 1 - 1).unwrap_or(&default) << 1) |
                (grid.try_get(x + 1 - 1, y + 1 - 1).unwrap_or(&default) << 0);
            *c = key[i];
            // if (x, y) == (2, 2) {
            //     println!("{} {}", i, key[i]);
            // }
        });

        grid = newgrid;

        // println!("{}", grid.map(|c| ['.', '#'][c]));
    }

    let mut part1 = 0;
    grid.for_each(|x, y, c| part1 += c);

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("20/input.txt").unwrap());
}
