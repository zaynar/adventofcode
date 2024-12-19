// Part 1: 55 mins
// Part 1+2: 56 mins

use std::collections::{HashSet, VecDeque};

use aocgrid::Grid;

#[derive(Clone, Debug)]
enum Re {
    N,E,S,W,
    Alt(Vec<Re>),
    Seq(Vec<Re>),
}

fn parse(input: &str) -> (Re, usize) {
    let mut ret = vec![Vec::new()];

    let mut consumed = 0;
    let mut chars = input.as_bytes();
    'OUTER: while consumed < chars.len() {
        let c = chars[consumed] as char;
        consumed += 1;
        // println!("# {}", c);
        match c {
            'N' => ret.last_mut().unwrap().push(Re::N),
            'E' => ret.last_mut().unwrap().push(Re::E),
            'S' => ret.last_mut().unwrap().push(Re::S),
            'W' => ret.last_mut().unwrap().push(Re::W),
            '(' => {
                let p = parse(&input[consumed..]);
                ret.last_mut().unwrap().push(p.0);
                consumed += p.1;
            }
            ')' => {
                break 'OUTER;
            }
            '|' => {
                ret.push(Vec::new());
            }
            _ => panic!()
        }
    }

    if ret.len() == 1 {
        return (Re::Seq(ret.first().unwrap().clone()), consumed);
    } else {
        return (Re::Alt(ret.iter().map(|r| Re::Seq(r.clone())).collect()), consumed);
    }
}

fn explore(grid: &mut Grid<char>, mut pos: HashSet<(i32, i32)>, re: &Re) -> HashSet<(i32, i32)> {

    match re {
        Re::N => {
            for (x, y) in &pos {
                grid.set(*x, *y - 1, '-');
            }
            pos = HashSet::from_iter(pos.iter().map(|(x, y)| (*x, *y - 2)));
        }
        Re::S => {
            for (x, y) in &pos {
                grid.set(*x, *y + 1, '-');
            }
            pos = HashSet::from_iter(pos.iter().map(|(x, y)| (*x, *y + 2)));
        }
        Re::W => {
            for (x, y) in &pos {
                grid.set(*x - 1, *y, '|');
            }
            pos = HashSet::from_iter(pos.iter().map(|(x, y)| (*x - 2, *y)));
        }
        Re::E => {
            for (x, y) in &pos {
                grid.set(*x + 1, *y, '|');
            }
            pos = HashSet::from_iter(pos.iter().map(|(x, y)| (*x + 2, *y)));
        }
        Re::Seq(vec) => {
            for v in vec {
                pos = explore(grid, pos, v);
            }
        }
        Re::Alt(vec) => {
            let mut newpos = HashSet::new();
            for v in vec {
                newpos = HashSet::from_iter(newpos.union(&explore(grid, pos.clone(), v)).cloned());
            }
            pos = newpos;
        }
    }

    // println!("{}", grid);

    pos
}

fn run(title: &str, input: &str) {
    let input = input.trim().strip_prefix("^").unwrap().strip_suffix("$").unwrap();

    // let sz = 17;
    let sz = 255;
    let mut grid = Grid::new_empty(sz, sz, '.');
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            grid.set(x, y, if x % 2 == 1 && y % 2 == 1 { '.' } else if x % 2 == 1 || y % 2 == 1 { '#' } else { '#'} );
        }
    }
    // println!("{}", grid);

    // println!("{:#?}", parse(input));

    let (x0, y0) = (grid.width() / 2 + 1, grid.height() / 2 + 1);
    let pos = HashSet::from([(x0, y0)]);
    grid.set(x0, y0, 'X');
    explore(&mut grid, pos, &parse(input).0);

    let mut open = VecDeque::new();
    open.push_back((x0, y0, 0));
    let mut seen = HashSet::new();
    let mut maxd = 0;
    let mut part2 = 0;
    while let Some((x, y, d)) = open.pop_front() {
        maxd = maxd.max(d);
        if !seen.insert((x, y)) {
            continue;
        }

        if d >= 1000 {
            part2 += 1;
        }

        if *grid.get(x - 1, y) == '|' {
            open.push_back((x - 2, y, d + 1));
        }
        if *grid.get(x + 1, y) == '|' {
            open.push_back((x + 2, y, d + 1));
        }
        if *grid.get(x, y - 1) == '-' {
            open.push_back((x, y - 2, d + 1));
        }
        if *grid.get(x, y + 1) == '-' {
            open.push_back((x, y + 2, d + 1));
        }
    }

    println!("{} part 1: {}", title, maxd - 1);
    println!("{} part 2: {}", title, part2);
}

fn main() {
    run("demo 1", "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
    run("demo 2", "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
    run("input", &std::fs::read_to_string("20/input.txt").unwrap());
}
