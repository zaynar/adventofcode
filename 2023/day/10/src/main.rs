use std::{fs, collections::VecDeque};

#[derive(Debug, Default, PartialEq)]
struct Pipe {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
}

const START: Pipe = Pipe { n: true, e: true, s: true, w: true };

fn decode_pipe(c: char) -> Pipe {
    let p = Pipe::default();
    match c {
        '|' => Pipe { n: true, s: true, ..p },
        '-' => Pipe { e: true, w: true, ..p },
        'L' => Pipe { n: true, e: true, ..p },
        'J' => Pipe { n: true, w: true, ..p },
        '7' => Pipe { s: true, w: true, ..p },
        'F' => Pipe { s: true, e: true, ..p },
        '.' => p,
        'S' => START,
        _ => unreachable!()
    }
}

fn main() {
    let mut grid: Vec<Vec<Pipe>> = fs::read_to_string("input").unwrap().lines().map(
        |line| line.chars().map(decode_pipe).collect()
    ).collect();

    let starts: Vec<_> = grid.iter().enumerate().map(
        |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| if *c == START { Some((x, y)) } else { None } ).collect::<Vec<_>>()
        }
    ).flatten().collect();

    assert!(starts.len() == 1);
    let (sx, sy) = starts[0];

    let sp = Pipe {
        n: if sy > 0 { grid[sy-1][sx].s } else { false },
        s: grid[sy+1][sx].n,
        w: if sx > 0 { grid[sy][sx-1].e } else { false },
        e: grid[sy][sx+1].w,
    };
    println!("{:?}", sp);
    grid[sy][sx] = sp;

    // println!("{:#?}", grid);

    let mut dist: Vec<Vec<Option<u32>>> = grid.iter().map(|row| row.iter().map(|_| None).collect()).collect();

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy, 0));
    while !queue.is_empty() {
        let (nx, ny, nd) = queue.pop_front().unwrap();
        if dist[ny][nx].is_some() {
            continue;
        }
        dist[ny][nx] = Some(nd);

        let p = &grid[ny][nx];
        if p.n {
            queue.push_back((nx, ny-1, nd+1));
        }
        if p.s {
            queue.push_back((nx, ny+1, nd+1));
        }
        if p.w {
            queue.push_back((nx-1, ny, nd+1));
        }
        if p.e {
            queue.push_back((nx+1, ny, nd+1));
        }
    }

    // println!("{:#?}", dist);
    println!("{}", dist.iter().flatten().filter_map(|n| *n).max().unwrap());

    let enclosed: Vec<Vec<_>> = dist.iter().enumerate().map(
        |(y, row)| {
            row.iter().enumerate().map(|(x, d)| {
                if d.is_some() {
                    return None;
                }
                let count = row[x+1..].iter().enumerate().filter(|(dx, n)| n.is_some() && grid[y][x+1+*dx].n).count();
                // println!("{:?} {}", row, count);
                if count % 2 == 0 {
                    return None;
                } else {
                    return Some(count);
                }
            }).collect::<Vec<_>>()
        }
    ).collect();
    for r in &enclosed {
        println!("{:?}", r.iter().map(|n| if n.is_some() { n.unwrap() } else { 0 }).collect::<Vec<_>>());
    }
    // println!("{:#?}", enclosed);
    println!("{:#?}", enclosed.iter().flatten().filter(|n| n.is_some()).count());

}
