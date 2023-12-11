use std::fs;

#[derive(Debug, PartialEq)]
enum Cell {
    Space,
    Galaxy,
}

fn decode_cell(c: char) -> Cell {
    match c {
        '.' => Cell::Space,
        '#' => Cell::Galaxy,
        _ => unreachable!(),
    }
}

fn main() {
    let mut grid: Vec<Vec<Cell>> = fs::read_to_string("input").unwrap().lines().map(
        |line| line.chars().map(decode_cell).collect()
    ).collect();

    let exp_rows: Vec<_> = grid.iter().enumerate().map(|(y, row)| {
        row.iter().all(|c| *c == Cell::Space)
    }).collect();

    let exp_cols: Vec<_> = (0..grid[0].len()).map(|x| {
        grid.iter().map(|row| &row[x]).all(|c| *c == Cell::Space)
    }).collect();

    let galaxies: Vec<_> = grid.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, c)| if *c == Cell::Galaxy { Some((x, y)) } else { None })
    }).flatten().collect();

    // println!("{:?}", grid);
    // println!("{:?}", exp_rows);
    // println!("{:?}", exp_cols);
    // println!("{:?}", galaxies);

    let mut ds = Vec::new();
    for i in 0..galaxies.len() {
        let gi = &galaxies[i];
        for j in (i+1)..galaxies.len() {
            let gj = &galaxies[j];

            let minx = gi.0.min(gj.0);
            let maxx = gi.0.max(gj.0);
            let miny = gi.1.min(gj.1);
            let maxy = gi.1.max(gj.1);

            // Part 1:
            // let dx: u64 = (minx..maxx).map(|x| if exp_cols[x] { 2 } else { 1 }).sum();
            // let dy: u64 = (miny..maxy).map(|y| if exp_rows[y] { 2 } else { 1 }).sum();

            // Part 2:
            let dx: u64 = (minx..maxx).map(|x| if exp_cols[x] { 1000000 } else { 1 }).sum();
            let dy: u64 = (miny..maxy).map(|y| if exp_rows[y] { 1000000 } else { 1 }).sum();

            let d = dx + dy;
            ds.push(d);
            // println!("{}", d);
        }
    }
    println!("Answer: {}", ds.iter().sum::<u64>());
}
