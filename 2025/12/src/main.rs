// Part 1: 23 mins
// Part 1+2: 23 mins

use aocgrid::Grid;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let segs = input.split("\n\n").collect_vec();
    let pieces = segs[0..=5].iter().map(|seg| {
        Grid::from(seg.split_once("\n").unwrap().1).map(|c| if c == '#' { 1 } else { 0 })
    }).collect_vec();

    let regions = segs[6].lines().map(|line| {
        let (sz, cs) = line.split_once(": ").unwrap();
        let (w, h) = sz.split("x").map(|n| n.parse::<u32>().unwrap()).collect_tuple().unwrap();
        let cs = cs.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect_vec();
        (w, h, cs)
    }).collect_vec();

    let piece_size = pieces.iter().map(|piece| {
        let mut n = 0;
        piece.for_each(|x, y, p| n += *p);
        n
    }).collect_vec();

    // println!("{pieces:?}");
    // println!("{regions:?}");

    let mut part1 = 0;

    for r in &regions {
        let (w, h, cs) = r;
        let used = cs.iter().enumerate().map(|(i, n)| piece_size[i] * n).sum::<u32>();
        if used > w * h {
            println!("{r:?} impossible");
        } else if (w / 3) * (h / 3) >= cs.iter().sum() {
            println!("{r:?} trivial");
            part1 += 1;
        } else {
            println!("{r:?}: {}", w * h - used);
        }
    }

    println!("{} part 1: {}", title, part1);
}

const INPUT_DEMO: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
