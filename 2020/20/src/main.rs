// Part 1: 36 mins
// Part 1+2: 58 mins

use std::collections::{HashMap, HashSet};

use aocgrid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    n: Vec<bool>,
    e: Vec<bool>,
    s: Vec<bool>,
    w: Vec<bool>,
    content: Grid<bool>,
}

fn flipv(mut t: Tile) -> Tile {
    t.e.reverse();
    t.w.reverse();
    (t.n, t.s) = (t.s, t.n);
    t.content = t.content.flipv();
    t
}

fn fliph(mut t: Tile) -> Tile {
    t.n.reverse();
    t.s.reverse();
    (t.e, t.w) = (t.w, t.e);
    t.content = t.content.fliph();
    t
}

fn rot90(mut t: Tile) -> Tile {
    (t.n, t.e, t.s, t.w) = (t.w, t.n, t.e, t.s);
    t.n.reverse();
    t.s.reverse();
    t.content = t.content.rot90();
    t
}

fn reorient_corner(edges: &HashMap<Vec<bool>, HashSet<u32>>, tile: Tile) -> Tile {
    for t in [
        tile.clone(),
        fliph(tile.clone()),
        flipv(tile.clone()),
        rot90(tile.clone()),
        fliph(rot90(tile.clone())),
        flipv(rot90(tile.clone())),
        rot90(rot90(tile.clone())),
        rot90(rot90(rot90(tile.clone())))
    ] {
        if edges.get(&t.n).unwrap().len() == 1 && edges.get(&t.w).unwrap().len() == 1 {
            return t;
        }
    }
    panic!();
}

fn reorient_w(e: &Vec<bool>, tile: Tile) -> Tile {
    for t in [
        tile.clone(),
        fliph(tile.clone()),
        flipv(tile.clone()),
        rot90(tile.clone()),
        fliph(rot90(tile.clone())),
        flipv(rot90(tile.clone())),
        rot90(rot90(tile.clone())),
        rot90(rot90(rot90(tile.clone())))
    ] {
        if &t.w == e {
            return t;
        }
    }
    panic!();
}

fn reorient_n(s: &Vec<bool>, tile: Tile) -> Tile {
    for t in [
        tile.clone(),
        fliph(tile.clone()),
        flipv(tile.clone()),
        rot90(tile.clone()),
        fliph(rot90(tile.clone())),
        flipv(rot90(tile.clone())),
        rot90(rot90(tile.clone())),
        rot90(rot90(rot90(tile.clone())))
    ] {
        if &t.n == s {
            return t;
        }
    }
    panic!();
}

fn run(title: &str, input: &str) {
    let mut tiles = HashMap::new();
    for tile in input.trim().split("\n\n") {
        let (id, grid) = tile.split_once("\n").unwrap();
        let id: u32 = id[5..9].parse().unwrap();
        let grid = Grid::from(grid);

        let n = (0..10).map(|i| *grid.get(i, 0) == '#').collect();
        let e = (0..10).map(|i| *grid.get(9, i) == '#').collect();
        let s = (0..10).map(|i| *grid.get(i, 9) == '#').collect();
        let w = (0..10).map(|i| *grid.get(0, i) == '#').collect();

        let tile = Tile { id, n, e, s, w, content: grid.map(|c| c == '#') };
        // println!("{:?}", tile);
        tiles.insert(id, tile);
    }

    let mut edges = HashMap::new();
    for (id, tile) in &tiles {
        for edge in [&tile.n, &tile.e, &tile.s, &tile.w] {
            edges.entry(edge.clone()).or_insert_with(|| HashSet::new()).insert(tile.id);

            let mut rev = edge.clone();
            rev.reverse();
            edges.entry(rev).or_insert_with(|| HashSet::new()).insert(tile.id);
        }
    }

    let mut corners = vec![];
    for (id, tile) in &tiles {
        let unique = [&tile.n, &tile.e, &tile.s, &tile.w].iter().filter(|&edge|
            edges.get(*edge).unwrap().len() == 1
        ).count();
        if unique == 2 {
            corners.push(id);
        }
        // println!("{} {}", tile.id, unique);
        // for t1 in &tiles {
        //     if t0.id == t1.id {
        //         continue;
        //     }
        // }
    }

    assert_eq!(corners.len(), 4);

    let sz = (tiles.len() as f64).sqrt() as usize;
    let mut placed = Grid::new_empty(sz, sz, None);
    placed.set(0, 0, Some(reorient_corner(&edges, tiles[corners[0]].clone())));

    for y in 0..placed.height() {
        for x in 0..placed.width() {
            if x == 0 && y == 0 {
                continue;
            }

            if y == 0 {
                let prev = placed.get(x - 1, y).clone().unwrap();
                let maybe = edges.get(&prev.e).unwrap().iter().filter(|id| **id != prev.id).collect_vec();
                assert_eq!(maybe.len(), 1);
                // println!("{} {} {:?}", prev, x, y, maybe);

                placed.set(x, y, Some(reorient_w(&prev.e, tiles[maybe[0]].clone())));
            } else {
                let prev = placed.get(x, y - 1).clone().unwrap();
                let maybe = edges.get(&prev.s).unwrap().iter().filter(|id| **id != prev.id).collect_vec();
                assert_eq!(maybe.len(), 1);
                placed.set(x, y, Some(reorient_n(&prev.s, tiles[maybe[0]].clone())));
            }
        }
    }

    for y in 0..placed.height() {
        for x in 0..placed.width() {
            print!("{} ", placed.get(x, y).clone().and_then(|t| Some(t.id)).unwrap_or(0));
        }
        println!();
    }

    let part1 =
    placed.get(0, 0).clone().unwrap().id as u64
    * placed.get(placed.width()-1, 0).clone().unwrap().id as u64
    * placed.get(placed.width()-1, placed.height()-1).clone().unwrap().id as u64
    * placed.get(0, placed.height()-1).clone().unwrap().id as u64;

    println!("{} part 1: {}", title, part1);

    let mut grid = Grid::new_empty(sz * 8, sz * 8, '.');
    for y in 0..placed.height() {
        for x in 0..placed.width() {
            let p = placed.get(x, y).clone().unwrap();
            for dy in 0..8 {
                for dx in 0..8 {
                    grid.set(x*8 + dx, y*8 + dy, if *p.content.get(1 + dx, 1 + dy) { '#' } else { '.' });
                }
            }
        }
    }

    let mut monster = Grid::from("                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ");

    for mut grid in [
        grid.clone(),
        grid.fliph(),
        grid.flipv(),
        grid.rot90(),
        grid.rot90().fliph(),
        grid.rot90().flipv(),
        grid.rot90().rot90(),
        grid.rot90().rot90().rot90(),
    ] {

        let mut found = 0;
        for y in 0..(grid.height() - monster.height()) {
            for x in 0..(grid.width() - monster.width()) {
                let mut ok = true;
                monster.for_each(|dx, dy, c| {
                    if *c == '#' && *grid.get(x + dx, y + dy) == '.' {
                        ok = false;
                    }
                });
                if ok {
                    found += 1;

                    monster.for_each(|dx, dy, c| {
                        if *c == '#' {
                            grid.set(x + dx, y + dy, 'O');
                        }
                    });

                }
            }
        }

        if found != 0 {
            // println!("{}", grid);
            println!("Found {}", found);

            let mut part2 = 0;
            grid.for_each(|x, y, c| if *c == '#' { part2 += 1 });
            println!("{} part 2: {}", title, part2);
        }
    }
    // let mut grid = Grid::new_empty(sz * 11, sz * 11, ' ');
    // for y in 0..placed.height() {
    //     for x in 0..placed.width() {
    //         let p = placed.get(x, y).clone().unwrap();
    //         for dy in 0..10 {
    //             for dx in 0..10 {
    //                 grid.set(x*11 + dx, y*11 + dy, if *p.content.get(dx, dy) { '#' } else { '.' });
    //             }
    //         }
    //     }
    // }

    // println!("{}", grid);

}

const INPUT_DEMO: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("20/input.txt").unwrap());
}
