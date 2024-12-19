// Part 1: 38 mins
// Part 1+2: 39 mins

use aocgrid::Grid;

fn run(title: &str, input: &str) {
    let mut grid = Grid::new_empty(2000, 2000, '.');

    let mut minx = grid.width();
    let mut maxx = 0;
    let mut miny = grid.height();
    let mut maxy = 0;

    for line in input.lines() {
        let (a, b) = line.split_once(", ").unwrap();
        let c: i32 = a[2..].parse().unwrap();
        let d = b[2..].split_once("..").unwrap();
        let e: i32 = d.0.parse().unwrap();
        let f: i32 = d.1.parse().unwrap();
        if a.starts_with(&"x") {
            let x = c;
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(e);
            maxy = maxy.max(f);
            for y in e..=f {
                grid.set(x, y, '#');
            }
        } else {
            let y = c;
            minx = minx.min(e);
            maxx = maxx.max(f);
            miny = miny.min(y);
            maxy = maxy.max(y);
            for x in e..=f {
                grid.set(x, y, '#');
            }
        }
    }

    let mut srcs = vec![(500, 0)];
    'OUTER: for i in 0..1000 {
        // println!("{:?}", srcs);
        for (srcx, srcy) in srcs.clone() {

            if "~".contains(*grid.get(srcx, srcy)) {
                continue;
            }

            for y in srcy..grid.height() {
                if ".|+".contains(*grid.get(srcx, y)) {
                    grid.set(srcx, y, '|');
                    continue;
                }

                if "#~".contains(*grid.get(srcx, y)) {
                    let mut x0 = srcx;
                    let mut x1 = srcx;
                    while "#~".contains(*grid.get(x0, y)) && ".|".contains(*grid.get(x0, y - 1)) {
                        x0 -= 1;
                    }
                    while "#~".contains(*grid.get(x1, y)) && ".|".contains(*grid.get(x1, y - 1)) {
                        x1 += 1;
                    }

                    // if i == 161 {
                    //     println!("{} {} - {} {}", x0, x1, srcx, y);
                    //     grid.set(srcx, y, '@');
                    //     break 'OUTER;
                    // }

                    if "#~".contains(*grid.get(x0, y)) && "#~".contains(*grid.get(x1, y)) {
                        for x in x0+1..=x1-1 {
                            grid.set(x, y - 1, '~');
                        }
                        srcs.push((srcx, srcy));
                    } else {
                        for x in x0+1..=x1-1 {
                            grid.set(x, y - 1, '|');
                        }

                        if !"#~".contains(*grid.get(x0, y)) && ".|".contains(*grid.get(x0, y - 1)) {
                            grid.set(x0, y - 1, '+');
                            srcs.push((x0, y - 1));
                        }
                        if !"#~".contains(*grid.get(x1, y)) && ".|".contains(*grid.get(x1, y - 1)) {
                            grid.set(x1, y - 1, '+');
                            srcs.push((x1, y - 1));
                        }

                    }
                }

                break;
            }

            srcs.sort();
            srcs.dedup();
        }
    }

    for y in miny-1..=maxy+1 {
        for x in minx-1..=maxx+1 {
            print!("{}", grid.get(x, y));
        }
        println!();
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for y in miny..=maxy {
        for x in minx-1..=maxx+1 {
            match grid.get(x, y) {
                '|' | '~' | '+' => part1 += 1,
                _ => (),
            }

            match grid.get(x, y) {
                '~' => part2 += 1,
                _ => (),
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("17/input.txt").unwrap());
}
