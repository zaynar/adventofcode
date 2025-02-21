use std::{fs, collections::HashMap};

fn sign(a: f64) -> f64 {
    if a == 0.0 {
        0.0
    } else {
        a.signum()
    }
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input").unwrap().lines().map(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let dir = tokens.next().unwrap().to_owned();
        let dist = tokens.next().unwrap().parse::<i32>().unwrap();
        let colour = tokens.next().unwrap()[2..8].to_owned();

        let dir2 = (match colour.chars().nth(5).unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => unreachable!(),
        }).to_owned();
        let dist2 = i64::from_str_radix(&colour[0..5], 16).unwrap();

        // (dir, dist)
        (dir2, dist2)
    }).collect();
    println!("{:?}", input);

    let mut verts = Vec::new();
    let mut hedges = Vec::new();
    let mut pos = (0, 0);
    verts.push(pos);
    for (dir, dist) in input {
        let (dx, dy) = match dir.as_str() {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        // let dist = if dx == 1 || dy == 1 { dist + 1 } else { dist - 1 };
        // if dx != 0 {
        //     hedges.push((pos.1, pos.0, pos.0 + dx * dist));
        // }
        pos = (pos.0 + dx * dist, pos.1 + dy * dist);
        verts.push(pos);
    }

    println!("{:?}", verts);

    let mut bpos = (0.0, 0.0);
    for (i, vs) in verts.iter().
    take(verts.len() - 1).
    cycle().take(verts.len() + 2).
    map(|(x, y)| (*x as f64, *y as f64)).collect::<Vec<_>>().windows(3)
    .enumerate() {
        let dir = -(
            (vs[2].0 - vs[1].0) * (vs[1].1 - vs[0].1) -
            (vs[2].1 - vs[1].1) * (vs[1].0 - vs[0].0)
        ).signum();
        // println!("{:?} {:?} {:?} {}", vs[0], vs[1], vs[2], dir);

        let mut npos = bpos;

        if dir > 0.0 {
            // println!("# {} + {} + {}", vs[1].0 + 0.5, sign(vs[1].0 - vs[0].0) * 0.5, sign(vs[1].0 - vs[2].0) * 0.5);
            npos = (
                vs[1].0 + 0.5 + sign(vs[1].0 - vs[0].0) * 0.5 + sign(vs[1].0 - vs[2].0) * 0.5,
                vs[1].1 + 0.5 + sign(vs[1].1 - vs[0].1) * 0.5 + sign(vs[1].1 - vs[2].1) * 0.5,
            )
            // if vs[0].1 == vs[1].1 {
            //     npos.0 += vs[1].0 - vs[0].0 + (vs[1].0 - vs[0].0).signum();
            // } else {
            //     npos.1 += vs[1].1 - vs[0].1 + (vs[1].1 - vs[0].1).signum();
            // }
            // if vs[0].1 == vs[1].1 {
            //     npos.0 += vs[1].0 - vs[0].0 + (vs[1].0 - vs[0].0).signum();
            // } else {
            //     npos.1 += vs[1].1 - vs[0].1 + (vs[1].1 - vs[0].1).signum();
            // }
        } else {
            npos = (
                vs[1].0 + 0.5 - sign(vs[1].0 - vs[0].0) * 0.5 - sign(vs[1].0 - vs[2].0) * 0.5,
                vs[1].1 + 0.5 - sign(vs[1].1 - vs[0].1) * 0.5 - sign(vs[1].1 - vs[2].1) * 0.5,
            )
            // if vs[0].1 == vs[1].1 {
            //     npos.0 += vs[1].0 - vs[0].0 - (vs[1].0 - vs[0].0).signum();
            // } else {
            //     npos.1 += vs[1].1 - vs[0].1 - (vs[1].1 - vs[0].1).signum();
            // }
        }

        if i > 0 && bpos.1 == npos.1 {
            hedges.push((bpos.1 as i64, bpos.0 as i64, npos.0 as i64));
        }

        println!("{:?}-{:?}", bpos, npos);
        bpos = npos;
    }

    hedges.sort();

    println!("{:?}", hedges);

    let mut area = 0;
    let mut active = 0;
    let mut prevy = 0;
    for (y, x0, x1) in hedges {
        area += active * (y as i64 - prevy);
        active += (x1 - x0) as i64;
        // area += (x1 - x0).abs();
        // area += ((x1 - x0).abs() - 1);
        // active += (x1 - x0).signum() * ((x1 - x0).abs() + 1);
        println!("{} {} {}", y, area, active);
        prevy = y as i64;
    }

    println!("{}", area);
}
